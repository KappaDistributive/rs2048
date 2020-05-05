#![allow(unused)]
use std::collections::HashSet;
use stdweb::traits::*;
use stdweb::web::{document, Element, INode};

use crate::canvas::Canvas;
use crate::scoreboard::Scoreboard;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    board: Vec<usize>,
    size: usize,
    score: usize,
    best: usize,
    finished: bool,
}

impl Game {
    pub fn new() -> Self {
        Game::from_size(4)
    }

    pub fn from_size(size: usize) -> Self {
        Game {
            board: vec![0; size * size],
            size,
            score: 0,
            best: 0,
            finished: false,
        }
    }

    /// This is a temporary implementation to generate a new cell and
    /// will be replaced once I've figured out how to generate random
    /// numbers in WASM.
    pub fn seed_cell(&mut self, seed: usize) {
        let mut candidates: Vec<usize> = Vec::new();
        for i in 0..self.board.len() {
            if self.board[i] == 0 {
                candidates.push(i);
            }
        }
        match seed % 9 {
            0 => {
                self.board[candidates[seed % candidates.len()]] = 4;
            }
            _ => {
                self.board[candidates[seed % candidates.len()]] = 2;
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn draw_board(&self, canvas: &Canvas) {
        canvas.clear_all();
        for y in 0..self.size {
            for x in 0..self.size {
                if self.get_state(x, y) != 0 {
                    canvas.draw_tile(
                        x,
                        y,
                        self.get_size(),
                        self.get_state(x, y),
                        self.foreground_color(x, y),
                        self.background_color(x, y),
                    );
                }
            }
        }
    }

    pub fn draw_score(&self, scoreboard: &Scoreboard) {
        scoreboard
            .scoreboard
            .set_text_content(&format!("{}", self.score));
        scoreboard.best.set_text_content(&format!("{}", self.best));
    }

    fn foreground_color(&self, x: usize, y: usize) -> &str {
        match self.get_state(x, y) {
            0 => &"#898077",
            2 => &"#898077",
            4 => &"#898077",
            8 => &"#f9f6f2",
            16 => &"#f9f6f2",
            32 => &"#f9f6f2",
            64 => &"#f9f6f2",
            128 => &"#f9f6f2",
            256 => &"#f9f6f2",
            512 => &"#f9f6f2",
            1024 => &"#f9f6f2",
            2048 => &"#f9f6f2",
            _ => &"#f9f6f2",
        }
    }

    fn background_color(&self, x: usize, y: usize) -> &str {
        match self.get_state(x, y) {
            0 => &"#f9f6f2",
            2 => &"#eee4da",
            4 => &"#ede0c8",
            8 => &"#f2b179",
            16 => &"#f59563",
            32 => &"#f67c5f",
            64 => &"#f65e3b",
            128 => &"#edcf72",
            256 => &"#edcc61",
            512 => &"#edc850",
            1024 => &"#edc53f",
            2048 => &"#edc22e",
            _ => &"#3c3a32",
        }
    }

    pub fn set_state(&mut self, x: usize, y: usize, value: usize) {
        if x < self.size && y < self.size {
            self.board[y * self.size + x] = value;
        } else {
            panic!("({},{}) is out ouf bounds!", x, y);
        }
    }

    pub fn get_state(&self, x: usize, y: usize) -> usize {
        if x < self.size && y < self.size {
            return self.board[y * self.size + x];
        }
        panic!("({},{}) is out ouf bounds!", x, y);
    }

    pub fn double_state(&mut self, x: usize, y: usize) {
        self.set_state(x, y, 2 * self.get_state(x, y));
    }

    pub fn set_states(&mut self, states: Vec<usize>) {
        self.board = states;
    }

    pub fn set_best(&mut self, best: usize) {
        self.best = best;
    }

    pub fn get_states(&self) -> Vec<usize> {
        self.board.clone()
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn get_best(&self) -> usize {
        self.best
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
                let value_c = self.get_state(x, y);
                let (x_t, y_t) = self.get_target(x, y, direction, &merged);
                let value_t = self.get_state(x_t, y_t);

                if value_c != 0 && (x != x_t || y != y_t) {
                    progress = true;
                    if value_c != value_t {
                        self.set_state(x_t, y_t, value_c);
                        self.set_state(x, y, 0);
                    } else {
                        merged.insert((x_t, y_t));
                        self.double_state(x_t, y_t);
                        self.set_state(x, y, 0);
                        self.score += self.get_state(x_t, y_t);
                    }
                }
            }
        }
        progress
    }

    /// Reset self.state, self.score and self.finished
    pub fn clear(&mut self) {
        self.board = vec![0; self.board.len()];
        self.best = std::cmp::max(self.best, self.score);
        self.score = 0;
        self.finished = false;
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
            && (self.get_state((a + vector.0) as usize, (b + vector.1) as usize) == 0
                || (self.get_state((a + vector.0) as usize, (b + vector.1) as usize)
                    == self.get_state(x, y)
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

impl std::fmt::Display for Game {
    /// Creates a text representation of self.state as in
    /// +-----------+-----------+-----------+-----------+
    /// |           |           |           |           |
    /// |           |           |           |           |
    /// |   32768   |           |           |           |
    /// |           |           |           |           |
    /// |           |           |           |           |
    /// +-----------+-----------+-----------+-----------+
    /// |           |           |           |           |
    /// |           |           |           |           |
    /// |           |           |           |           |
    /// |           |           |           |           |
    /// |           |           |           |           |
    /// +-----------+-----------+-----------+-----------+
    /// |           |           |           |           |
    /// |           |           |           |           |
    /// |           |           |   2048    |           |
    /// |           |           |           |           |
    /// |           |           |           |           |
    /// +-----------+-----------+-----------+-----------+
    /// |           |           |           |           |
    /// |           |           |           |           |
    /// |           |    128    |           |    256    |
    /// |           |           |           |           |
    /// |           |           |           |           |
    /// +-----------+-----------+-----------+-----------+
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let print_width: usize = 1 + self.size * 12;
        let print_height: usize = 1 + self.size * 6;
        let mut temp: Vec<char> = Vec::new();

        // create empty board with cell borders
        for y in 0..print_height {
            for x in 0..print_width {
                match (y % 6, x % 12) {
                    (0, 0) => temp.push('+'),
                    (0, _) => temp.push('-'),
                    (_, 0) => temp.push('|'),
                    _ => temp.push(' '),
                }
            }
            temp.push('\n');
        }

        // fill in cells with proper offset
        for y in 0..self.size {
            for x in 0..self.size {
                if self.get_state(x, y) > 0 {
                    // get cell value
                    let cell_state: Vec<char> = self
                        .get_state(x, y)
                        .to_string()
                        .chars()
                        .collect::<Vec<char>>();
                    let cell_len = cell_state.len();
                    for (z, &st) in cell_state.iter().enumerate() {
                        // calculate offset
                        let offset = 6 - cell_len / 2;
                        let dest = (3 + y * 6) * (print_width + 1)
                            + x * 12 + offset + z;
                        temp[dest] = st;
                    }
                }
            }
        }
        write!(f, "{}", temp.iter().collect::<String>())
    }
}
