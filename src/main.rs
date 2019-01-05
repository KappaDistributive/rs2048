#[macro_use]
extern crate stdweb;

mod canvas;
mod game;
mod ran;
use crate::canvas::Canvas;
use crate::game::Direction;
use crate::game::Game;

use std::cell::RefCell;
use std::rc::Rc;

use stdweb::traits::*;
use stdweb::web::{
    document, event::KeyDownEvent, event::MouseDownEvent, event::MouseUpEvent, IEventTarget,
};

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_data(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }
}

fn get_direction(last: Point, current: Point) -> Direction {
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

fn main() {
    stdweb::initialize();
    let mut index: usize = 1;
    let last_mouse_pos = Rc::new(RefCell::new(Point::from_data(0, 0)));
    let game = Rc::new(RefCell::new(Game::new()));
    let canvas = Canvas::new("#canvas");

    // Initialize game
    game.borrow_mut().seed_cell(ran::RAN[0]);
    game.borrow().draw_board(&canvas);

    // Add event handler MouseDown
    document()
        .get_element_by_id("canvas")
        .unwrap()
        .add_event_listener({
            let last_mouse_pos = last_mouse_pos.clone();
            move |event: MouseDownEvent| {
                event.prevent_default();
                last_mouse_pos
                    .borrow_mut()
                    .set(event.client_x(), event.client_y());
            }
        });

    // Add event handler MouseUp
    document()
        .get_element_by_id("canvas")
        .unwrap()
        .add_event_listener({
            let game = game.clone();
            let last_mouse_pos = last_mouse_pos.clone();
            move |event: MouseUpEvent| {
                event.prevent_default();
                let mut progress: bool = false;
                let current_mouse_pos = Point::from_data(event.client_x(), event.client_y());
                let last_mouse_pos = Point::from_data(
                    last_mouse_pos.borrow().get_x(),
                    last_mouse_pos.borrow().get_y(),
                );
                let direction = get_direction(last_mouse_pos, current_mouse_pos);
                progress = game.borrow_mut().step(&direction);
                if progress {
                    game.borrow_mut().seed_cell(ran::RAN[index % 10000]);
                    index += 1;
                }
            }
        });

    // Add event handler KeyDown
    document().add_event_listener({
        let game = game.clone();
        move |event: KeyDownEvent| {
            #[allow(unused_mut)]
            let mut progress: bool;
            console!(log, "key down!");
            match event.key().as_ref() {
                "ArrowUp" => {
                    progress = game.borrow_mut().step(&Direction::Up);
                }
                "ArrowDown" => {
                    progress = game.borrow_mut().step(&Direction::Down);
                }
                "ArrowLeft" => {
                    progress = game.borrow_mut().step(&Direction::Left);
                }
                "ArrowRight" => {
                    progress = game.borrow_mut().step(&Direction::Right);
                }
                "r" => {
                    game.borrow_mut().clear();
                    progress = true;
                }
                _ => {
                    progress = false;
                }
            };
            if progress {
                game.borrow_mut().seed_cell(ran::RAN[index % 10000]);
                index += 1;
            }
        }
    });

    // Redraw board every 100ms
    fn game_loop(game: Rc<RefCell<Game>>, canvas: Rc<Canvas>, time: u32) {
        stdweb::web::set_timeout(
            move || {
                game_loop(game.clone(), canvas.clone(), time);
                game.borrow().draw_board(&canvas);
            },
            time,
        );
    }

    // Initiate first draw
    game_loop(game, Rc::new(canvas), 100);

    stdweb::event_loop();
}
