use crate::piece::Tetromino;
use rand::prelude::*;

pub struct Bag {
    queue: Vec<Tetromino>,
    rng: ThreadRng,
}

impl Bag {
    pub fn new() -> Self {
        let mut bag = Self {
            queue: Vec::new(),
            rng: rand::thread_rng(),
        };
        bag.refill();
        bag
    }

    pub fn next(&mut self) -> Tetromino {
        if self.queue.is_empty() {
            self.refill();
        }
        self.queue.pop().unwrap()
    }

    fn refill(&mut self) {
        let mut pieces = vec![
            Tetromino::I,
            Tetromino::O,
            Tetromino::T,
            Tetromino::S,
            Tetromino::Z,
            Tetromino::J,
            Tetromino::L,
        ];
        pieces.shuffle(&mut self.rng);
        self.queue = pieces;
    }
}
