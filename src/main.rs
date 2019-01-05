#[macro_use]
extern crate stdweb;
use stdweb::traits::*;
use stdweb::web::{document, event};

use std::sync::{Arc, Mutex};

mod canvas;
mod game;
mod scoreboard;
mod util;

use crate::canvas::Canvas;
use crate::game::Direction;
use crate::game::Game;
use crate::scoreboard::Scoreboard;
use crate::util::*;

fn main() {
    // Initialize framework
    stdweb::initialize();

    // Game state
    let mut index: usize = 1;
    let mut game = Game::new();
    let canvas = Canvas::new("#canvas");
    let scoreboard = Scoreboard::new("#scoreboard");
    let mut last_mouse_pos = Point::from_data(0, 0);

    // Initialize game
    game.seed_cell(get_seed());
    game.draw_board(&canvas);
    game.draw_score(&scoreboard);

    // Process a single GameEvent
    let process_event_fn = move |game_event| {
        let progress = match game_event {
            GameEvent::MouseDown(event) => {
                last_mouse_pos.set(event.client_x(), event.client_y());
                false
            }
            GameEvent::MouseUp(event) => {
                let current_mouse_pos = Point::from_data(event.client_x(), event.client_y());
                let direction = get_direction(&last_mouse_pos, &current_mouse_pos);
                game.step(&direction)
            }
            GameEvent::KeyDown(event) => match event.key().as_ref() {
                "ArrowUp" => game.step(&Direction::Up),
                "ArrowDown" => game.step(&Direction::Down),
                "ArrowLeft" => game.step(&Direction::Left),
                "ArrowRight" => game.step(&Direction::Right),
                "r" => {
                    game.clear();
                    true
                }
                _ => false,
            },
        };
        if progress {
            game.seed_cell(get_seed());
            index += 1;
            game.draw_score(&scoreboard);
            game.draw_board(&canvas);
        }
    };

    // The event processing closure needs to be mutably
    // shared between event handlers. Interior mutability
    // will work.
    let process_event = Arc::new(Mutex::new(Box::new(process_event_fn)));

    // Add event handler MouseDown
    document()
        .get_element_by_id("canvas")
        .unwrap()
        .add_event_listener({
            let process_event = process_event.clone();
            move |event: event::MouseDownEvent| {
                event.prevent_default();
                let ref mut process_event = *process_event.lock().unwrap();
                process_event(GameEvent::MouseDown(event));
            }
        });

    // Add event handler MouseUp
    document()
        .get_element_by_id("canvas")
        .unwrap()
        .add_event_listener({
            let process_event = process_event.clone();
            move |event: event::MouseUpEvent| {
                let ref mut process_event = *process_event.lock().unwrap();
                event.prevent_default();
                process_event(GameEvent::MouseUp(event));
            }
        });

    // Add event handler KeyDown
    document().add_event_listener({
        let process_event = process_event.clone();
        move |event: event::KeyDownEvent| {
            let ref mut process_event = *process_event.lock().unwrap();
            process_event(GameEvent::KeyDown(event));
        }
    });

    // Start the event loop (which will never return)
    stdweb::event_loop();
}
