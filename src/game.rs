use crate::Board;
use crate::Direction;
use crate::Renderer;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

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

    pub fn draw(&self, renderer: &Renderer) {
        renderer.clear_all();

        // draw grid
        for x in 1..self.size {
            renderer.draw_rect(
                f64::from(x as u32) * f64::from(renderer.canvas.width())
                    / f64::from(self.size as u32),
                0.0,
                1.0,
                f64::from(renderer.canvas.height()),
                &"black",
            );
        }
        for y in 1..self.size {
            renderer.draw_rect(
                0.0,
                f64::from(y as u32) * f64::from(renderer.canvas.height())
                    / f64::from(self.size as u32),
                f64::from(renderer.canvas.width()),
                1.0,
                &"black",
            );
        }

        for y in 0..self.size {
            for x in 0..self.size {
                if self.board.get_state(x, y) != 0 {
                    renderer.draw_rect(
                        f64::from(x as u32) * f64::from(renderer.canvas.width())
                            / f64::from(self.size as u32),
                        f64::from(y as u32) * f64::from(renderer.canvas.height())
                            / f64::from(self.size as u32),
                        f64::from(renderer.canvas.width()) / f64::from(self.size as u32),
                        f64::from(renderer.canvas.height()) / f64::from(self.size as u32),
                        &"black",
                    );
                }
            }
        }
    }

    pub fn print(&self) {
        self.board.print();
    }

    pub fn to_string(&self) -> String {
        self.board.to_string()
    }

    pub fn set_state(&mut self, x: usize, y: usize, value: usize) {
        self.board.set_state(x, y, value);
    }

    pub fn set_states(&mut self, states: Vec<usize>) {
        self.board.set_states(states);
    }

    pub fn get_states(&self) -> Vec<usize> {
        self.board.get_states()
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn step(&mut self, direction: &Direction) -> bool {
        let mut progress: bool = false;
        let (x_transversal, y_transversal) = self.build_transveral(direction);
        let mut merged: std::collections::HashSet<(usize, usize)> =
            std::collections::HashSet::new();
        for y in y_transversal {
            for x in x_transversal.clone() {
                let value_c = self.board.get_state(x, y);
                let (x_t, y_t) = self.get_target(x, y, direction, &merged);
                let value_t = self.board.get_state(x_t, y_t);

                if value_c != 0 && (x != x_t || y != y_t) {
                    progress = true;
                    if value_c != value_t {
                        self.board.set_state(x_t, y_t, value_c);
                        self.board.set_state(x, y, 0);
                    } else {
                        merged.insert((x_t, y_t));
                        self.board.double_state(x_t, y_t);
                        self.board.set_state(x, y, 0);
                    }
                }
            }
        }
        progress
    }

    /// Reset self.state, self.score and self.finished
    pub fn clear(&mut self) {
        self.board = Board::from_size(self.size);
        self.score = 0;
        self.finished = false;
    }

    /// Fill a new random cell randomly with either 2 or 4
    pub fn generate_new_cell(&mut self) {
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
            let rad: f32 = rng.gen();
            match rad < 0.9 {
                true => self.board.set_state(x, y, 2),
                false => self.board.set_state(x, y, 4),
            }
        }
    }

    // private helper functions

    /// Return position of cell to be merged with or moved to (x,y)
    fn get_target(
        &self,
        x: usize,
        y: usize,
        direction: &Direction,
        merged: &HashSet<(usize, usize)>,
    ) -> (usize, usize) {
        let mut a = x as i32;
        let mut b = y as i32;
        let vector = self.get_vector(direction);
        while self.legal_position(a + vector.0, b + vector.1)
            && (self
                .board
                .get_state((a + vector.0) as usize, (b + vector.1) as usize)
                == 0
                || (self
                    .board
                    .get_state((a + vector.0) as usize, (b + vector.1) as usize)
                    == self.board.get_state(x, y)
                    && !merged.contains(&((a + vector.0) as usize, (b + vector.1) as usize))))
        {
            a += vector.0;
            b += vector.1;
        }
        (a as usize, b as usize)
    }

    /// Check whether (x,y) is a position in self.board.state
    fn legal_position(&self, x: i32, y: i32) -> bool {
        0 <= x && x < self.size as i32 && 0 <= y && y < self.size as i32
    }

    /// Build a list of positions to traverse in the right order
    fn build_transveral(&self, direction: &Direction) -> (Vec<usize>, Vec<usize>) {
        let mut x_transversal: Vec<usize> = Vec::new();
        let mut y_transversal: Vec<usize> = Vec::new();
        for i in 0..self.size {
            x_transversal.push(i);
            y_transversal.push(i);
        }
        match direction {
            Direction::Right => x_transversal.reverse(),
            Direction::Down => y_transversal.reverse(),
            _ => {}
        }
        (x_transversal, y_transversal)
    }

    /// Get vector of direction to move in
    fn get_vector(&self, direction: &Direction) -> (i32, i32) {
        match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        }
    }
}
