use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    style::{Color, Print, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::error::Error;
use std::io::BufRead;
use std::io::{stdout, Write};
use std::path;
use std::time;

pub struct Config {
    pub init_state_file: path::PathBuf,
    pub refresh_rate_usec: u64,
}

impl Config {
    pub fn new(init_state_file: path::PathBuf, refresh_rate_usec: u64) -> Config {
        Config {
            init_state_file,
            refresh_rate_usec,
        }
    }
}

struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn new(x: u16, y: u16) -> Point {
        Point { x, y }
    }
}

struct GameBoard {
    width: u16,
    height: u16,
    points: Vec<bool>,
}

impl GameBoard {
    fn new(width: u16, height: u16, init_state: &Vec<Point>) -> GameBoard {
        let mut points = vec![false; usize::from(width * height)];
        for point in init_state {
            points[usize::from(point.x + point.y * width)] = true;
        }
        GameBoard {
            width,
            height,
            points,
        }
    }

    fn count_live_neighbors(&self, x: u16, y: u16) -> u16 {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let x = (x as isize + i) as u16;
                let y = (y as isize + j) as u16;
                if x < self.width && y < self.height && self.points[usize::from(x + y * self.width)]
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn next_state(&mut self) {
        self.points = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .enumerate()
            .map(|(idx, (x, y))| {
                let count = self.count_live_neighbors(x, y);
                matches!(
                    (self.points[idx], count),
                    (true, 2) | (true, 3) | (false, 3)
                )
            })
            .collect();
    }

    fn draw(&self) -> Result<(), Box<dyn Error>> {
        let mut stdout = stdout();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = usize::from(x + y * self.width);
                let ch = if self.points[idx] { '█' } else { ' ' };

                stdout.execute(SetForegroundColor(Color::White))?;
                stdout.execute(MoveTo(x, y))?;
                stdout.execute(Print(ch))?;
            }
        }
        Ok(())
    }
}

fn load_initial_state(init_state_file: &path::PathBuf) -> Result<Vec<Point>, std::io::Error> {
    let mut points = Vec::new();
    let file = std::fs::File::open(init_state_file)?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let coords = line
            .trim()
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .collect::<Vec<_>>();

        if coords.len() == 2 {
            if let (Ok(x), Ok(y)) = (
                coords[0].trim().parse::<u16>(),
                coords[1].trim().parse::<u16>(),
            ) {
                points.push(Point::new(x, y));
            }
        }
    }

    Ok(points)
}

pub fn run_draw_loop(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let screen_dim = crossterm::terminal::size()?;
    let mut game_board = GameBoard::new(
        screen_dim.0,
        screen_dim.1 - 1, // Leave some space for the quit message.
        &load_initial_state(&config.init_state_file)?,
    );

    // Enter raw mode, alternate screen, clear it, and hide the cursor.
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(Hide)?;

    // Add the text "Press 'q' to quit" to the bottom of the screen.
    stdout.execute(MoveTo(0, screen_dim.1 - 1))?;
    stdout.execute(SetForegroundColor(Color::White))?;
    stdout.execute(Print("press 'q' to quit"))?;

    // Main game loop
    loop {
        // Update and draw game state
        game_board.next_state();
        game_board.draw()?;
        stdout.flush()?;

        // Check for quit command
        if event::poll(time::Duration::from_millis(config.refresh_rate_usec))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char('q') || key_event.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }

    // Reset terminal state before exit.
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
