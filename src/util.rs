use stdweb;
use stdweb::unstable::TryInto;

use crate::game::Direction;

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn from_data(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

pub fn get_direction(last: &Point, current: &Point) -> Direction {
    if (last.x - current.x).abs() > (last.y - current.y).abs() {
        // move horizontal
        return if last.x > current.x {
            Direction::Left
        } else {
            Direction::Right
        };
    } else {
        // move vertical
        return if last.y > current.y {
            Direction::Up
        } else {
            Direction::Down
        };
    }
}

pub fn get_seed() -> usize {
    let rand = js! { return Math.random(); };
    let base: f64 = rand.try_into().unwrap();
    (base * 1000000 as f64).floor() as usize
}
