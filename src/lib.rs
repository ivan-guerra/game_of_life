use std::io::BufRead;
use std::path;

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
