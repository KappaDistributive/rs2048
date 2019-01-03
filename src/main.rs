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
use stdweb::web::{document, event::KeyDownEvent, IEventTarget};

fn main() {
    stdweb::initialize();
    let mut index: usize = 1;
    let game = Rc::new(RefCell::new(Game::new()));
    game.borrow_mut().seed_cell(ran::RAN[0]);
    //let pre = document().query_selector(&"#pre").unwrap().unwrap();
    let canvas = Canvas::new("#canvas");

    game.borrow().draw_board(&canvas);

    document().add_event_listener({
        //game.clone();
        move |event: KeyDownEvent| {
            #[allow(unused_mut)]
            let mut progress: bool;
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
                game.borrow().draw_board(&canvas);
                //pre.set_text_content(&game.borrow().to_string());
            }
        }
    });

    stdweb::event_loop();
}
