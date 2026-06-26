use std::time::{Duration, Instant};

use crate::bag::Bag;
use crate::board::Board;
use crate::constants::*;
use crate::piece::{TetrisColor, Tetromino};

#[derive(Clone)]
pub struct Piece {
    pub kind: Tetromino,
    pub x: i32,
    pub y: i32,
    pub rotation: usize,
}

impl Piece {
    pub fn new(kind: Tetromino) -> Self {
        Self {
            x: kind.spawn_x(),
            y: kind.spawn_y(),
            rotation: 0,
            kind,
        }
    }

    pub fn cells(&self) -> [(i32, i32); 4] {
        let offsets = self.kind.cells(self.rotation);
        let mut result = [(0, 0); 4];
        for (i, &(ox, oy)) in offsets.iter().enumerate() {
            result[i] = (self.x + ox, self.y + oy);
        }
        result
    }

    pub fn color(&self) -> TetrisColor {
        self.kind.color()
    }

    pub fn rotated_cells(&self, new_rotation: usize) -> [(i32, i32); 4] {
        let offsets = self.kind.cells(new_rotation);
        let mut result = [(0, 0); 4];
        for (i, &(ox, oy)) in offsets.iter().enumerate() {
            result[i] = (self.x + ox, self.y + oy);
        }
        result
    }
}

pub struct Game {
    pub board: Board,
    bag: Bag,
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub score: u32,
    pub level: u32,
    pub lines: u32,
    pub game_over: bool,

    last_tick: Instant,
    lock_start: Option<Instant>,
    grounded: bool,
    lock_resets: u32,
}

impl Game {
    pub fn new() -> Self {
        let mut bag = Bag::new();
        let first = bag.next();
        let second = bag.next();
        Self {
            board: Board::new(),
            current_piece: Piece::new(first),
            next_piece: Piece::new(second),
            score: 0,
            level: 1,
            lines: 0,
            game_over: false,
            last_tick: Instant::now(),
            lock_start: None,
            grounded: false,
            lock_resets: 0,
            bag,
        }
    }

    fn tick_interval(&self) -> Duration {
        let ms = (TICK_BASE_MS as f64 * 0.8_f64.powf((self.level - 1) as f64)) as u64;
        Duration::from_millis(ms.max(TICK_MIN_MS))
    }

    pub fn tick(&mut self) {
        if self.game_over {
            return;
        }

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);

        if elapsed >= self.tick_interval() {
            self.last_tick = now;
            self.apply_gravity();
        }

        // Check lock delay
        if self.grounded {
            if let Some(lock_start) = self.lock_start {
                if now.duration_since(lock_start) >= Duration::from_millis(LOCK_DELAY_MS) {
                    self.lock_piece();
                }
            }
        }
    }

    fn apply_gravity(&mut self) {
        let next_y = self.current_piece.y + 1;
        let cells = self.current_piece.cells();
        let test_cells = cells.map(|(x, y)| (x, y + 1));

        if self.board.collides(&test_cells) {
            if !self.grounded {
                self.grounded = true;
                self.lock_start = Some(Instant::now());
            }
        } else {
            self.current_piece.y = next_y;
            self.grounded = false;
            self.lock_start = None;
        }
    }

    fn lock_piece(&mut self) {
        let cells = self.current_piece.cells();
        let color = self.current_piece.color();
        self.board.lock_cells(&cells, color);

        let cleared = self.board.clear_lines();
        if cleared > 0 {
            let idx = (cleared - 1).min(3);
            self.score += BASE_SCORE[idx] * self.level;
            self.lines += cleared as u32;
            self.level = self.lines / LINES_PER_LEVEL + 1;
        }

        self.spawn_next();
        self.grounded = false;
        self.lock_start = None;
        self.lock_resets = 0;
    }

    fn spawn_next(&mut self) {
        let next_kind = self.bag.next();
        self.current_piece = std::mem::replace(&mut self.next_piece, Piece::new(next_kind));
        self.last_tick = Instant::now();

        if self.board.collides(&self.current_piece.cells()) {
            self.game_over = true;
        }
    }

    pub fn move_left(&mut self) {
        if self.game_over {
            return;
        }
        let cells = self.current_piece.cells();
        let test = cells.map(|(x, y)| (x - 1, y));
        if !self.board.collides(&test) {
            self.current_piece.x -= 1;
            self.reset_lock_if_grounded();
        }
    }

    pub fn move_right(&mut self) {
        if self.game_over {
            return;
        }
        let cells = self.current_piece.cells();
        let test = cells.map(|(x, y)| (x + 1, y));
        if !self.board.collides(&test) {
            self.current_piece.x += 1;
            self.reset_lock_if_grounded();
        }
    }

    pub fn rotate_cw(&mut self) {
        if self.game_over {
            return;
        }
        self.try_rotate((self.current_piece.rotation + 1) % 4);
    }

    pub fn rotate_ccw(&mut self) {
        if self.game_over {
            return;
        }
        self.try_rotate((self.current_piece.rotation + 3) % 4);
    }

    fn try_rotate(&mut self, new_rotation: usize) {
        let kicks = self
            .current_piece
            .kind
            .wall_kicks(self.current_piece.rotation, new_rotation);

        for &(dx, dy) in kicks.iter() {
            let test = self.current_piece.rotated_cells(new_rotation);
            let test: [(i32, i32); 4] = test.map(|(x, y)| (x + dx, y + dy));
            if !self.board.collides(&test) {
                self.current_piece.rotation = new_rotation;
                self.current_piece.x += dx;
                self.current_piece.y += dy;
                self.reset_lock_if_grounded();
                return;
            }
        }
    }

    pub fn soft_drop(&mut self) {
        if self.game_over {
            return;
        }
        let cells = self.current_piece.cells();
        let test = cells.map(|(x, y)| (x, y + 1));
        if !self.board.collides(&test) {
            self.current_piece.y += 1;
            self.score += SOFT_DROP_SCORE;
            self.grounded = false;
            self.lock_start = None;
        }
    }

    pub fn hard_drop(&mut self) {
        if self.game_over {
            return;
        }
        let mut drop_distance = 0;
        loop {
            let cells = self.current_piece.cells();
            let test = cells.map(|(x, y)| (x, y + drop_distance + 1));
            if self.board.collides(&test) {
                break;
            }
            drop_distance += 1;
        }
        self.current_piece.y += drop_distance;
        self.score += HARD_DROP_SCORE * drop_distance as u32;
        self.lock_piece();
    }

    fn reset_lock_if_grounded(&mut self) {
        if !self.grounded {
            return;
        }
        let cells = self.current_piece.cells();
        let test = cells.map(|(x, y)| (x, y + 1));
        if !self.board.collides(&test) {
            self.grounded = false;
            self.lock_start = None;
            return;
        }
        if self.lock_resets < MAX_LOCK_RESETS {
            self.lock_start = Some(Instant::now());
            self.lock_resets += 1;
        }
    }

    /// Returns the ghost piece position (where the piece would land if hard-dropped).
    pub fn ghost_y(&self) -> i32 {
        let mut dy = 0;
        loop {
            let cells = self.current_piece.cells();
            let test = cells.map(|(x, y)| (x, y + dy + 1));
            if self.board.collides(&test) {
                break;
            }
            dy += 1;
        }
        self.current_piece.y + dy
    }
}
