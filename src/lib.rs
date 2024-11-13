use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event},
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

#[derive(Debug, PartialEq)]
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

fn center_points_on_screen(points: &[Point], screen_width: u16, screen_height: u16) -> Vec<Point> {
    if points.is_empty() {
        return Vec::new();
    }

    // Convert screen dimensions to f32 once at the start
    let screen_width_f = f32::from(screen_width);
    let screen_height_f = f32::from(screen_height);

    // Find the bounding box of the input points
    let min_x = points
        .iter()
        .map(|p| f32::from(p.x))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_x = points
        .iter()
        .map(|p| f32::from(p.x))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let min_y = points
        .iter()
        .map(|p| f32::from(p.y))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_y = points
        .iter()
        .map(|p| f32::from(p.y))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // Calculate pattern dimensions
    let pattern_width = max_x - min_x;
    let pattern_height = max_y - min_y;

    // Calculate scaling factors if pattern is larger than screen
    let scale_x = if pattern_width >= screen_width_f {
        (screen_width_f - 2.0) / pattern_width
    } else {
        1.0
    };
    let scale_y = if pattern_height >= screen_height_f {
        (screen_height_f - 2.0) / pattern_height
    } else {
        1.0
    };
    // Use the smaller scale to maintain aspect ratio
    let scale = scale_x.min(scale_y);

    // Calculate scaled dimensions
    let scaled_width = pattern_width * scale;
    let scaled_height = pattern_height * scale;

    // Calculate offsets to center the scaled pattern
    let x_offset = (screen_width_f - scaled_width) / 2.0 - (min_x * scale);
    let y_offset = (screen_height_f - scaled_height) / 2.0 - (min_y * scale);

    // Translate and scale all points, keeping them within screen bounds
    points
        .iter()
        .map(|p| {
            let scaled_x = f32::from(p.x) * scale + x_offset;
            let scaled_y = f32::from(p.y) * scale + y_offset;
            Point::new(
                (scaled_x.floor() as u16).min(screen_width.saturating_sub(1)),
                (scaled_y.floor() as u16).min(screen_height.saturating_sub(1)),
            )
        })
        .collect()
}

pub fn run_draw_loop(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let screen_dim = crossterm::terminal::size()?;
    let init_state = center_points_on_screen(
        &load_initial_state(&config.init_state_file)?,
        screen_dim.0,
        screen_dim.1,
    );
    let mut game_board = GameBoard::new(screen_dim.0, screen_dim.1, &init_state);

    // Enter raw mode, alternate screen, clear it, and hide the cursor.
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(Hide)?;

    // Main game loop
    loop {
        // Update and draw game state
        game_board.next_state();
        game_board.draw()?;
        stdout.flush()?;

        // Check for any keypress
        if event::poll(time::Duration::from_micros(config.refresh_rate_usec))? {
            if let Event::Key(_) = event::read()? {
                break;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use testdir::testdir;

    #[test]
    fn new_game_board_constructs_with_valid_initial_state() {
        let init_state = vec![Point::new(1, 1)];
        let board = GameBoard::new(3, 3, &init_state);

        assert_eq!(board.width, 3);
        assert_eq!(board.height, 3);
        assert_eq!(board.points.len(), 9);
        assert!(!board.points[0]); // (0,0) empty
        assert!(board.points[4]); // (1,1) alive
    }

    #[test]
    fn count_live_neighbors_counts_zero_on_empty_board() {
        let init_state = vec![];
        let board = GameBoard::new(3, 3, &init_state);

        assert_eq!(board.count_live_neighbors(1, 1), 0);
    }

    #[test]
    fn count_live_neighbors_counts_one_on_single_cell_board() {
        let init_state = vec![Point { x: 0, y: 0 }];
        let board = GameBoard::new(3, 3, &init_state);

        assert_eq!(board.count_live_neighbors(1, 1), 1);
    }

    #[test]
    fn count_live_neighbors_on_full_board() {
        let init_state = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 2 },
        ];
        let board = GameBoard::new(3, 3, &init_state);

        assert_eq!(board.count_live_neighbors(1, 1), 8);
    }

    #[test]
    fn count_live_neighbors_counts_edge_neighbors() {
        let init_state = vec![Point { x: 0, y: 0 }, Point { x: 1, y: 0 }];
        let board = GameBoard::new(3, 3, &init_state);

        assert_eq!(board.count_live_neighbors(0, 0), 1);
    }

    #[test]
    fn count_live_neighbors_counts_corner_neighbors() {
        let init_state = vec![
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
        ];
        let board = GameBoard::new(3, 3, &init_state);
        assert_eq!(board.count_live_neighbors(0, 0), 3);
    }

    #[test]
    fn count_live_neighbors_does_not_count_self() {
        let init_state = vec![Point { x: 1, y: 1 }];
        let board = GameBoard::new(3, 3, &init_state);
        assert_eq!(board.count_live_neighbors(1, 1), 0);
    }

    #[test]
    fn next_state_cell_dies_from_underpopulation() {
        // Single cell dies from loneliness.
        let init_state = vec![Point { x: 1, y: 1 }];
        let mut board = GameBoard::new(3, 3, &init_state);
        board.next_state();

        assert!(!board.points[4]); // Center cell should die
    }

    #[test]
    fn next_state_cell_survives() {
        // Three cells in a row, middle cell should survive.
        let init_state = vec![
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
        ];
        let mut board = GameBoard::new(3, 3, &init_state);
        board.next_state();

        assert!(board.points[4]); // Center cell should survive
    }

    #[test]
    fn next_state_cell_dies_from_overpopulation() {
        // Center cell surrounded by 4 neighbors should die.
        let init_state = vec![
            Point { x: 0, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 2, y: 2 },
        ];
        let mut board = GameBoard::new(3, 3, &init_state);
        board.next_state();

        assert!(!board.points[4]); // Center cell should die
    }

    #[test]
    fn next_state_cell_is_born_through_reproduction() {
        // Empty cell with exactly 3 neighbors should become alive.
        let init_state = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
        ];
        let mut board = GameBoard::new(3, 3, &init_state);
        board.next_state();

        assert!(board.points[4]); // Center cell should become alive
    }

    #[test]
    fn load_initial_state_can_load_empty_file() {
        let dir = testdir!();
        let file_path = dir.join("empty.txt");
        fs::write(&file_path, "").unwrap();

        let points = load_initial_state(&file_path).unwrap();
        assert!(points.is_empty());
    }

    #[test]
    fn load_initial_state_can_load_file_with_one_point() {
        let dir = testdir!();
        let file_path = dir.join("single.txt");
        fs::write(&file_path, "(1,2)\n").unwrap();

        let points = load_initial_state(&file_path).unwrap();
        assert_eq!(points.len(), 1);
        assert_eq!(points[0], Point::new(1, 2));
    }

    #[test]
    fn load_initial_state_can_load_file_with_multiple_points() {
        let dir = testdir!();
        let file_path = dir.join("multiple.txt");
        fs::write(&file_path, "(1,2)\n(3,4)\n(5,6)\n").unwrap();

        let points = load_initial_state(&file_path).unwrap();
        assert_eq!(points.len(), 3);
        assert_eq!(points[0], Point::new(1, 2));
        assert_eq!(points[1], Point::new(3, 4));
        assert_eq!(points[2], Point::new(5, 6));
    }

    #[test]
    fn load_initial_state_ignores_lines_with_invalid_format() {
        let dir = testdir!();
        let file_path = dir.join("invalid_format.txt");
        fs::write(&file_path, "(1,2)\ninvalid\n(3,4)\n").unwrap();

        let points = load_initial_state(&file_path).unwrap();
        assert_eq!(points.len(), 2);
        assert_eq!(points[0], Point::new(1, 2));
        assert_eq!(points[1], Point::new(3, 4));
    }

    #[test]
    fn load_initial_state_ignores_invalid_numbers() {
        let dir = testdir!();
        let file_path = dir.join("invalid_numbers.txt");
        fs::write(&file_path, "(1,2)\n(a,b)\n(3,4)\n").unwrap();

        let points = load_initial_state(&file_path).unwrap();
        assert_eq!(points.len(), 2);
        assert_eq!(points[0], Point::new(1, 2));
        assert_eq!(points[1], Point::new(3, 4));
    }

    #[test]
    fn load_initial_state_can_handle_extra_whitespace() {
        let dir = testdir!();
        let file_path = dir.join("whitespace.txt");
        fs::write(&file_path, "  (1, 2)  \n\t(3,4)\t\n").unwrap();

        let points = load_initial_state(&file_path).unwrap();
        assert_eq!(points.len(), 2);
        assert_eq!(points[0], Point::new(1, 2));
        assert_eq!(points[1], Point::new(3, 4));
    }

    #[test]
    fn load_initial_state_returns_err_on_nonexistent_file() {
        let dir = testdir!();
        let file_path = dir.join("nonexistent.txt");

        assert!(load_initial_state(&file_path).is_err());
    }

    #[test]
    fn center_points_on_screen_returns_empty_vec_when_given_no_points() {
        let points = Vec::new();
        let centered = center_points_on_screen(&points, 10, 10);

        assert!(centered.is_empty());
    }

    #[test]
    fn center_points_on_screen_centers_single_point() {
        let points = vec![Point::new(0, 0)];
        let centered = center_points_on_screen(&points, 3, 3);

        assert_eq!(centered, vec![Point::new(1, 1)]);
    }

    #[test]
    fn center_points_on_screen_centers_pattern_smaller_than_screen() {
        let points = vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(1, 1),
        ];
        let centered = center_points_on_screen(&points, 10, 10);

        let expected = vec![
            Point::new(4, 4),
            Point::new(5, 4),
            Point::new(4, 5),
            Point::new(5, 5),
        ];
        assert_eq!(centered, expected);
    }

    #[test]
    fn center_points_on_screen_centers_pattern_larger_than_screen() {
        let points = vec![
            Point::new(0, 0),
            Point::new(10, 0),
            Point::new(0, 10),
            Point::new(10, 10),
        ];
        let centered = center_points_on_screen(&points, 5, 5);

        // Pattern should be scaled down to fit
        assert!(centered.iter().all(|p| p.x < 5 && p.y < 5));
        // Should maintain relative positions
        assert!(centered[1].x > centered[0].x);
        assert!(centered[2].y > centered[0].y);
    }

    #[test]
    fn center_points_on_screen_maintains_aspect_ratio() {
        let points = vec![
            Point::new(0, 0),
            Point::new(4, 0),
            Point::new(0, 2),
            Point::new(4, 2),
        ];
        let centered = center_points_on_screen(&points, 10, 10);

        // Check that width:height ratio is maintained
        let width = centered[1].x - centered[0].x;
        let height = centered[2].y - centered[0].y;
        assert_eq!(width / 2, height);
    }

    #[test]
    fn center_points_on_screen_respects_screen_boundaries() {
        let points = vec![Point::new(0, 0), Point::new(100, 100)];
        let screen_width = 5;
        let screen_height = 5;
        let centered = center_points_on_screen(&points, screen_width, screen_height);

        for point in centered {
            assert!(point.x < screen_width);
            assert!(point.y < screen_height);
        }
    }

    #[test]
    fn center_points_on_screen_preserves_scaling() {
        let points = vec![Point::new(0, 0), Point::new(2, 0), Point::new(0, 1)];
        let centered = center_points_on_screen(&points, 20, 20);

        // Original 2:1 ratio should be maintained
        let horizontal_dist = centered[1].x - centered[0].x;
        let vertical_dist = centered[2].y - centered[0].y;

        assert_eq!(horizontal_dist, vertical_dist * 2);
    }
}
