pub struct Board {
    size: usize,
    state: Vec<usize>,
}

impl Board {
    pub fn new() -> Self {
        Board::from_size(4)
    }

    pub fn from_size(size: usize) -> Self {
        Board {
            size: size,
            state: vec![0usize; size * size],
        }
    }

    /// return self.state corresponding to (x,y)
    pub fn get_state(&self, x: usize, y: usize) -> usize {
        if x < self.size && y < self.size {
            self.state[y * self.size + x]
        } else {
            panic!("({},{}) are out of bounds!", x, y);
        }
    }

    /// set self.state at (x,y) to value
    pub fn set_state(&mut self, x:usize, y:usize, value: usize) {
        if x < self.size && y < self.size {
            self.state[y * self.size + x] = value;
        } else {
            panic!("({},{}) are out of bounds!", x, y);
        }
    }

    /// double self.state at (x,y)
    pub fn double_state(&mut self, x: usize, y:usize) {
        if x < self.size && y < self.size {
            self.state[y * self.size + x] *= 2;
        } else {
            panic!("({},{}) are out of bounds!", x, y);
        }
    }
    /// Print a representation of self.state as in
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
    pub fn print(&self) {
        print!("{}", self.to_string());
    }

    /// Creates a String representation of self.state as in
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
    pub fn to_string(&self) -> String {
        let print_width: usize = 1 + self.size * 12;
        let print_height: usize = 1 + self.size * 6;
        let mut temp: Vec<char> = Vec::new();

        // create empty board
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

                    for z in 0..cell_len {
                        // calculate offset
                        let offset = 6 - cell_len / 2;
                        temp[(3 + y * 6) * (print_width + 1) + x * 12 + offset + z] = cell_state[z];
                    }
                }
            }
        }
        temp.iter().collect()
    }

    // private helper functions
}
