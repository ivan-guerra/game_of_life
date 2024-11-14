//! Conway's Game of Life
//!
//! This module provides a terminal-based visualization of Conway's Game of Life,
//! a cellular automaton that follows simple rules to create complex patterns.
//!
//! # Features
//!
//! - Terminal-based visualization using crossterm.
//! - Configurable refresh rate.
//! - Load initial patterns from text files.
//! - Automatic centering and scaling of patterns to fit the terminal.
//! - Raw terminal mode for smooth rendering.
//!
//! # Rules
//!
//! The game follows Conway's classic rules:
//! 1. Any live cell with 2 or 3 live neighbors survives.
//! 2. Any dead cell with exactly 3 live neighbors becomes alive.
//! 3. All other cells die or remain dead.
//!
//! # Example
//!
//! ```no_run
//! use game_of_life::{Config, run_draw_loop};
//!
//! let config = Config::new(
//!     std::path::PathBuf::from("patterns/glider.txt"),
//!     100_000, // 100ms refresh rate
//! );
//! run_draw_loop(&config).expect("Failed to run game");
//! ```
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

/// Configuration for the Game of Life simulation.
///
/// Contains settings for initialization and execution of the simulation,
/// including the path to the initial state file and refresh rate.
pub struct Config {
    /// Path to the file containing the initial state of the game grid.
    pub init_state_file: path::PathBuf,
    /// Refresh rate of the game simulation in microseconds.
    pub refresh_rate_usec: u64,
}

impl Config {
    /// Creates a new Config instance with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `init_state_file` - Path to the file containing the initial state configuration.
    /// * `refresh_rate_usec` - The refresh rate of the game simulation in microseconds.
    ///
    /// # Returns
    ///
    /// A new Config instance initialized with the provided parameters.
    pub fn new(init_state_file: path::PathBuf, refresh_rate_usec: u64) -> Config {
        Config {
            init_state_file,
            refresh_rate_usec,
        }
    }
}

/// Represents a point in 2D space with unsigned integer coordinates.
///
/// Used to specify cell positions in the Game of Life grid, where:
/// - x represents the horizontal position.
/// - y represents the vertical position.
#[derive(Debug, PartialEq)]
pub struct Point {
    /// The horizontal coordinate (column).
    pub x: u16,
    /// The vertical coordinate (row).
    pub y: u16,
}

impl Point {
    /// Creates a new Point with the given x and y coordinates.
    pub fn new(x: u16, y: u16) -> Point {
        Point { x, y }
    }
}

/// Represents the game board for Conway's Game of Life.
///
/// Contains the dimensions of the board and the current state of all cells,
/// where each cell is represented as a boolean value (true for alive, false for dead).
pub struct GameBoard {
    /// Width of the game board in cells.
    pub width: u16,
    /// Height of the game board in cells.
    pub height: u16,
    /// Vector storing the state of each cell, where true represents a live cell
    /// and false represents a dead cell. The cells are stored in row-major order.
    pub points: Vec<bool>,
}

impl GameBoard {
    /// Creates a new GameBoard with the specified dimensions and initial state.
    pub fn new(width: u16, height: u16, init_state: &Vec<Point>) -> GameBoard {
        let mut points = vec![false; usize::from(width) * usize::from(height)];
        for point in init_state {
            points[usize::from(point.x + point.y * width)] = true;
        }
        GameBoard {
            width,
            height,
            points,
        }
    }

    /// Counts the number of live neighbors for a cell at the specified coordinates.
    /// # Returns
    ///
    /// The number of live neighboring cells (0-8) for the cell at position (x, y).
    pub fn count_live_neighbors(&self, x: u16, y: u16) -> u16 {
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

    /// Calculates and updates the next state of the game board according to Conway's Game of Life rules.
    ///
    /// The rules are:
    /// 1. Any live cell with 2 or 3 live neighbors survives.
    /// 2. Any dead cell with exactly 3 live neighbors becomes alive.
    /// 3. All other cells die or remain dead.
    pub fn next_state(&mut self) {
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

    /// Draws the current state of the game board to the terminal.
    ///
    /// Uses crossterm to:
    /// - Move the cursor to each cell position.
    /// - Set the text color to white.
    /// - Print either a full block character ('█') for live cells or a space for dead cells.
    ///
    /// # Returns
    ///
    /// `Result<(), Box<dyn Error>>` - Ok if drawing succeeds, Err if there's a terminal error.
    pub fn draw(&self) -> Result<(), Box<dyn Error>> {
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

/// Loads the initial state of the game board from a file.
///
/// # Arguments
///
/// * `init_state_file` - Path to the file containing initial cell coordinates.
///
/// # Returns
///
/// * `Result<Vec<Point>, std::io::Error>` - A vector of Points representing live cells if successful,
///   or an IO error if file operations fail.
///
/// # Format
///
/// The file should contain coordinates in the format "(x,y)" with one coordinate pair per line.
pub fn load_initial_state(init_state_file: &path::PathBuf) -> Result<Vec<Point>, std::io::Error> {
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

/// Centers and scales a collection of points to fit within the screen dimensions.
///
/// # Arguments
///
/// * `points` - A slice of Points to be centered and scaled.
/// * `screen_width` - The width of the screen in characters.
/// * `screen_height` - The height of the screen in characters.
///
/// # Returns
///
/// A new vector of Points that have been centered and scaled to fit the screen while
/// maintaining aspect ratio. Returns an empty vector if the input is empty.
///
/// # Details
///
/// The function:
/// 1. Finds the bounding box of all input points.
/// 2. Calculates appropriate scaling to fit the screen while maintaining aspect ratio.
/// 3. Centers the scaled points on screen.
/// 4. Ensures all points remain within screen bounds.
pub fn center_points_on_screen(
    points: &[Point],
    screen_width: u16,
    screen_height: u16,
) -> Vec<Point> {
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

/// Runs the main game loop for Conway's Game of Life with terminal visualization.
///
/// # Arguments
///
/// * `config` - Configuration settings for the game, including initial state file and refresh
///              rate.
///
/// # Returns
///
/// `Result<(), Box<dyn Error>>` - Ok if the game runs and exits successfully, Err if there's an error.
///
/// # Details
///
/// This function:
/// 1. Initializes the terminal in raw mode with an alternate screen.
/// 2. Loads and centers the initial game state.
/// 3. Runs the main game loop until a key is pressed.
/// 4. Handles terminal cleanup before exit.
///
/// The game loop:
/// - Updates the game state according to Conway's rules.
/// - Draws the current state to the terminal.
/// - Checks for key presses to exit.
/// - Maintains the specified refresh rate.
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
