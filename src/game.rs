use crate::Board;
use crate::Direction;
use rand::{thread_rng, Rng};

pub struct Game {
    board: Board,
    size: usize,
    score: usize,
    finished: bool,
}

impl Game {
    pub fn new() -> Self {
        Game::from_size(4)
    }

    pub fn from_size(size: usize) -> Self {
        Game {
            board: Board::from_size(size),
            size: size,
            score: 0,
            finished: false,
        }
    }
    pub fn init(&mut self) {
        self.generate_new_cell();
    }

    pub fn is_finished(self) -> bool {
        self.finished
    }

    pub fn print(&self) {
        self.board.print();
    }

    pub fn to_string(&self) -> String {
        self.board.to_string()
    }

    pub fn step(&mut self, direction: &Direction) {
        if self.merge(direction) || self.glide(direction) {
            self.generate_new_cell();
        }
    }

    // private helper functions

    /// reset self.state, self.score and self.finished
    fn clear(&mut self) {
        self.board = Board::from_size(self.size);
        self.score = 0;
        self.finished = false;
    }
    /// fills a new random cell randomly with either 2 or 4
    fn generate_new_cell(&mut self) {
        let mut candidates: Vec<(usize, usize)> = Vec::new();
        for y in 0..self.size {
            for x in 0..self.size {
                if self.board.get_state(x, y) == 0 {
                    candidates.push((x, y));
                }
            }
        }
        let candidates_len = candidates.len();
        if candidates_len == 0 {
            panic!("Game has ended!");
        } else {
            let mut rng = thread_rng();
            let ran: usize = rng.gen_range(0, candidates_len);
            let (x, y) = candidates[ran];
            match rng.gen() {
                true => self.board.set_state(x, y, 2),
                false => self.board.set_state(x, y, 4),
            }
        }
    }

    fn merge(&mut self, direction: &Direction) -> bool {
        let mut progress: bool = false;
        match *direction {
            Direction::Up => {
                for x in 0..self.size {
                    for y in 0..self.size {
                        if self.board.get_state(x, y) != 0 {
                            for z in y + 1..self.size {
                                if self.board.get_state(x, y) == self.board.get_state(x, z) {
                                    progress = true;
                                    self.board.double_state(x, y);
                                    self.board.set_state(x, z, 0);
                                }
                            }
                        }
                    }
                }
            }
            Direction::Down => {
                for x in 0..self.size {
                    for y in (0..self.size).rev() {
                        if self.board.get_state(x, y) != 0 {
                            for z in (0..y).rev() {
                                if self.board.get_state(x, y) == self.board.get_state(x, z) {
                                    progress = true;
                                    self.board.double_state(x, y);
                                    self.board.set_state(x, z, 0);
                                }
                            }
                        }
                    }
                }
            }
            Direction::Left => {
                for y in 0..self.size {
                    for x in 0..self.size {
                        if self.board.get_state(x, y) != 0 {
                            for z in x + 1..self.size {
                                if self.board.get_state(x, y) == self.board.get_state(z, y) {
                                    progress = true;
                                    self.board.double_state(x, y);
                                    self.board.set_state(z, y, 0);
                                }
                            }
                        }
                    }
                }
            }
            Direction::Right => {
                for y in 0..self.size {
                    for x in (0..self.size).rev() {
                        if self.board.get_state(x, y) != 0 {
                            for z in (0..x).rev() {
                                if self.board.get_state(x, y) == self.board.get_state(z, y) {
                                    progress = true;
                                    self.board.double_state(x, y);
                                    self.board.set_state(z, y, 0);
                                }
                            }
                        }
                    }
                }
            }
        }
        progress
    }

    fn glide(&mut self, direction: &Direction) -> bool {
        let mut progress: bool = false;

        match *direction {
            Direction::Up => {
                for x in 0..self.size {
                    for y in 1..self.size {
                        if self.board.get_state(x, y) != 0 {
                            let mut z = y;
                            let mut is_moved: bool = false;
                            while z > 0 && self.board.get_state(x, z - 1) == 0 {
                                z -= 1;
                                is_moved = true;
                            }
                            if is_moved {
                                progress = true;
                                self.board.set_state(x, z, self.board.get_state(x, y));
                                self.board.set_state(x, y, 0);
                            }
                        }
                    }
                }
            }
            Direction::Down => {
                for x in 0..self.size {
                    for y in (0..self.size - 1).rev() {
                        if self.board.get_state(x, y) != 0 {
                            let mut z = y;
                            let mut is_moved: bool = false;
                            while z + 1 < self.size && self.board.get_state(x, z + 1) == 0 {
                                z += 1;
                                is_moved = true;
                            }
                            if is_moved {
                                progress = true;
                                self.board.set_state(x, z, self.board.get_state(x, y));
                                self.board.set_state(x, y, 0);
                            }
                        }
                    }
                }
            }
            Direction::Left => {
                for y in 0..self.size {
                    for x in 1..self.size {
                        if self.board.get_state(x, y) != 0 {
                            let mut z = x;
                            let mut is_moved: bool = false;
                            while z > 0 && self.board.get_state(z - 1, y) == 0 {
                                z -= 1;
                                is_moved = true;
                            }
                            if is_moved {
                                progress = true;
                                self.board.set_state(z, y, self.board.get_state(x, y));
                                self.board.set_state(x, y, 0);
                            }
                        }
                    }
                }
            }
            Direction::Right => {
                for y in 0..self.size {
                    for x in (0..self.size).rev() {
                        if self.board.get_state(x, y) != 0 {
                            let mut z = x;
                            let mut is_moved: bool = false;
                            while z + 1 < self.size && self.board.get_state(z + 1, y) == 0 {
                                z += 1;
                                is_moved = true;
                            }
                            if is_moved {
                                progress = true;
                                self.board.set_state(z, y, self.board.get_state(x, y));
                                self.board.set_state(x, y, 0);
                            }
                        }
                    }
                }
            }
        }
        progress
    }
}
