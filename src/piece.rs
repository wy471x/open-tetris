use crate::constants::BOARD_COLS;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tetromino {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Clone, Copy, Debug)]
pub struct TetrisColor(pub u8, pub u8, pub u8);

impl Tetromino {
    pub fn cells(self, rotation: usize) -> [(i32, i32); 4] {
        match self {
            Tetromino::I => [
                [(0, 1), (1, 1), (2, 1), (3, 1)],
                [(2, 0), (2, 1), (2, 2), (2, 3)],
                [(0, 2), (1, 2), (2, 2), (3, 2)],
                [(1, 0), (1, 1), (1, 2), (1, 3)],
            ][rotation],
            Tetromino::O => [
                [(0, 0), (1, 0), (0, 1), (1, 1)],
                [(0, 0), (1, 0), (0, 1), (1, 1)],
                [(0, 0), (1, 0), (0, 1), (1, 1)],
                [(0, 0), (1, 0), (0, 1), (1, 1)],
            ][rotation],
            Tetromino::T => [
                [(0, 1), (1, 1), (2, 1), (1, 0)],
                [(1, 0), (1, 1), (2, 1), (1, 2)],
                [(0, 1), (1, 1), (2, 1), (1, 2)],
                [(0, 1), (1, 1), (1, 0), (1, 2)],
            ][rotation],
            Tetromino::S => [
                [(0, 1), (1, 0), (1, 1), (2, 0)],
                [(1, 0), (1, 1), (2, 1), (2, 2)],
                [(0, 2), (1, 1), (1, 2), (2, 1)],
                [(0, 0), (0, 1), (1, 1), (1, 2)],
            ][rotation],
            Tetromino::Z => [
                [(0, 0), (1, 0), (1, 1), (2, 1)],
                [(2, 0), (1, 1), (2, 1), (1, 2)],
                [(0, 1), (1, 1), (1, 2), (2, 2)],
                [(1, 0), (0, 1), (1, 1), (0, 2)],
            ][rotation],
            Tetromino::J => [
                [(0, 0), (0, 1), (1, 1), (2, 1)],
                [(1, 0), (1, 1), (1, 2), (2, 0)],
                [(0, 1), (1, 1), (2, 1), (2, 2)],
                [(0, 2), (1, 0), (1, 1), (1, 2)],
            ][rotation],
            Tetromino::L => [
                [(2, 0), (0, 1), (1, 1), (2, 1)],
                [(1, 0), (1, 1), (1, 2), (2, 2)],
                [(0, 1), (1, 1), (2, 1), (0, 2)],
                [(0, 0), (1, 0), (1, 1), (1, 2)],
            ][rotation],
        }
    }

    pub fn color(self) -> TetrisColor {
        match self {
            Tetromino::I => TetrisColor(0, 240, 240),
            Tetromino::O => TetrisColor(240, 240, 0),
            Tetromino::T => TetrisColor(160, 0, 240),
            Tetromino::S => TetrisColor(0, 240, 0),
            Tetromino::Z => TetrisColor(240, 0, 0),
            Tetromino::J => TetrisColor(0, 0, 240),
            Tetromino::L => TetrisColor(240, 160, 0),
        }
    }

    pub fn spawn_x(self) -> i32 {
        match self {
            Tetromino::O => BOARD_COLS as i32 / 2 - 1,
            _ => BOARD_COLS as i32 / 2 - 2,
        }
    }

    pub fn spawn_y(self) -> i32 {
        0
    }

    /// Wall kick offsets for (from_rotation, to_rotation). Returns 5 test offsets.
    pub fn wall_kicks(self, from: usize, to: usize) -> &'static [(i32, i32); 5] {
        match self {
            Tetromino::I => &WALL_KICKS_I[from * 4 + to],
            Tetromino::O => &WALL_KICKS_O,
            _ => &WALL_KICKS_JLSTZ[from * 4 + to],
        }
    }
}

type KickTable = [[(i32, i32); 5]; 16];

const WALL_KICKS_JLSTZ: KickTable = {
    let mut t = [[(0, 0); 5]; 16];
    // 0->1
    t[1] = [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)];
    // 1->0
    t[4] = [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)];
    // 1->2
    t[6] = [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)];
    // 2->1
    t[9] = [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)];
    // 2->3
    t[11] = [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)];
    // 3->2
    t[14] = [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)];
    // 3->0
    t[12] = [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)];
    // 0->3
    t[3] = [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)];
    t
};

const WALL_KICKS_I: KickTable = {
    let mut t = [[(0, 0); 5]; 16];
    // 0->1
    t[1] = [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)];
    // 1->0
    t[4] = [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)];
    // 1->2
    t[6] = [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)];
    // 2->1
    t[9] = [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)];
    // 2->3
    t[11] = [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)];
    // 3->2
    t[14] = [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)];
    // 3->0
    t[12] = [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)];
    // 0->3
    t[3] = [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)];
    t
};

const WALL_KICKS_O: [(i32, i32); 5] = [(0, 0); 5];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_pieces_have_4_cells() {
        for kind in &[Tetromino::I, Tetromino::O, Tetromino::T, Tetromino::S, Tetromino::Z, Tetromino::J, Tetromino::L] {
            for rot in 0..4 {
                let cells = kind.cells(rot);
                assert_eq!(cells.len(), 4, "{kind:?} rot {rot}");
            }
        }
    }

    #[test]
    fn test_o_piece_never_changes() {
        let r0 = Tetromino::O.cells(0);
        for rot in 1..4 {
            assert_eq!(r0, Tetromino::O.cells(rot));
        }
    }
}
