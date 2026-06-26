use crate::constants::{BOARD_COLS, BOARD_ROWS};
use crate::piece::TetrisColor;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub occupied: bool,
    pub color: TetrisColor,
}

impl Cell {
    pub fn empty() -> Self {
        Self {
            occupied: false,
            color: TetrisColor(0, 0, 0),
        }
    }
}

pub struct Board {
    pub grid: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: vec![vec![Cell::empty(); BOARD_COLS]; BOARD_ROWS],
        }
    }

    pub fn collides(&self, cells: &[(i32, i32); 4]) -> bool {
        for &(x, y) in cells {
            if x < 0 || x >= BOARD_COLS as i32 || y >= BOARD_ROWS as i32 {
                return true;
            }
            if y < 0 {
                continue;
            }
            if self.grid[y as usize][x as usize].occupied {
                return true;
            }
        }
        false
    }

    pub fn lock_cells(&mut self, cells: &[(i32, i32); 4], color: TetrisColor) {
        for &(x, y) in cells {
            if y < 0 || y >= BOARD_ROWS as i32 || x < 0 || x >= BOARD_COLS as i32 {
                continue;
            }
            self.grid[y as usize][x as usize] = Cell {
                occupied: true,
                color,
            };
        }
    }

    /// Clears completed lines and returns the number of lines cleared.
    pub fn clear_lines(&mut self) -> usize {
        let mut cleared = 0;
        let mut row = BOARD_ROWS as i32 - 1;
        while row >= 0 {
            if self.grid[row as usize].iter().all(|c| c.occupied) {
                self.grid.remove(row as usize);
                self.grid.insert(0, vec![Cell::empty(); BOARD_COLS]);
                cleared += 1;
            } else {
                row -= 1;
            }
        }
        cleared
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_board_no_collision() {
        let board = Board::new();
        assert!(!board.collides(&[(3, 0), (4, 0), (5, 0), (4, 1)]));
    }

    #[test]
    fn test_left_wall_collision() {
        let board = Board::new();
        assert!(board.collides(&[(-1, 0), (0, 0), (1, 0), (0, 1)]));
    }

    #[test]
    fn test_right_wall_collision() {
        let board = Board::new();
        assert!(board.collides(&[(9, 0), (10, 0), (11, 0), (10, 1)]));
    }

    #[test]
    fn test_bottom_collision() {
        let board = Board::new();
        assert!(board.collides(&[(3, 19), (4, 19), (5, 19), (4, 20)]));
    }

    #[test]
    fn test_above_board_no_collision() {
        let board = Board::new();
        assert!(!board.collides(&[(3, -2), (4, -2), (5, -2), (4, -1)]));
    }

    #[test]
    fn test_lock_and_clear_lines() {
        let mut board = Board::new();
        for col in 0..BOARD_COLS {
            board.grid[BOARD_ROWS - 1][col].occupied = true;
        }
        assert_eq!(board.clear_lines(), 1);
        assert!(!board.grid[BOARD_ROWS - 1][0].occupied);
    }

    #[test]
    fn test_clear_multiple_lines() {
        let mut board = Board::new();
        for row in (BOARD_ROWS - 2)..BOARD_ROWS {
            for col in 0..BOARD_COLS {
                board.grid[row][col].occupied = true;
            }
        }
        assert_eq!(board.clear_lines(), 2);
    }
}
