pub const BOARD_COLS: usize = 10;
pub const BOARD_ROWS: usize = 20;

pub const TICK_BASE_MS: u64 = 800;
pub const TICK_MIN_MS: u64 = 50;
pub const LOCK_DELAY_MS: u64 = 500;
pub const MAX_LOCK_RESETS: u32 = 15;

// score = BASE_SCORE[lines_cleared - 1] * level
pub const BASE_SCORE: [u32; 4] = [100, 300, 500, 800];
pub const LINES_PER_LEVEL: u32 = 10;
pub const SOFT_DROP_SCORE: u32 = 1;
pub const HARD_DROP_SCORE: u32 = 2;
