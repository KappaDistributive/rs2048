use stdweb::unstable::TryInto;
use stdweb::web::event;

use crate::game::Direction;

pub enum GameEvent {
    KeyDown(event::KeyDownEvent),
    MouseDown(event::MouseDownEvent),
    MouseUp(event::MouseUpEvent),
    Tick,
}

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
        if last.x > current.x {
            Direction::Left
        } else {
            Direction::Right
        }
    } else {
        // move vertical
        if last.y > current.y {
            Direction::Up
        } else {
            Direction::Down
        }
    }
}

pub fn get_seed() -> usize {
    let rand = js! { return Math.random(); };
    let base: f64 = rand.try_into().unwrap();
    (base * 1_000_000.0).floor() as usize
}
