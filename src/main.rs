#[macro_use]
extern crate stdweb;
use stdweb::traits::*;
use stdweb::web::{document, event, window, confirm};

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

// Set this to some positive number of milliseconds
// to get a game tick at that interval.
const TICK_MS: Option<u32> = None;

fn main() {
    // Initialize framework
    stdweb::initialize();

    // Game state
    let mut index: usize = 1;
    let mut game = Game::new();
    let canvas = Canvas::new("#canvas");
    let scoreboard = Scoreboard::new("#scoreboard", "#best");
    let mut last_mouse_pos = Point::from_data(0, 0);

    // Initialize game

    // Attempt to recover previous best from web storage
    // XXX Will be silent on fetch / parse failure.
    if let Some(s) = window().local_storage().get(&"best") {
        if let Ok(best) = s.parse::<usize>() {
            game.set_best(best);
        }
    }

    game.seed_cell(get_seed());
    game.draw_board(&canvas);
    game.draw_score(&scoreboard);

    // End initialization of game

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
                    if confirm("Reset game?") {
                        game.clear();
                        game.save_best();
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
            GameEvent::Tick => {
                // XXX Enable TICK_MS above and
                // uncomment below to try out the
                // interval timer.

                // canvas.clear_all();
                false
            }
            GameEvent::Exit => {
                game.clear();
                game.save_best();
                false
            }
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
    let process_event: Arc<Mutex<dyn FnMut(GameEvent)>> = Arc::new(Mutex::new(process_event_fn));

    // Add event handler MouseDown
    document()
        .get_element_by_id("canvas")
        .unwrap()
        .add_event_listener({
            let process_event = process_event.clone();
            move |event: event::MouseDownEvent| {
                event.prevent_default();
                let process_event = &mut *process_event.lock().unwrap();
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
                let process_event = &mut *process_event.lock().unwrap();
                event.prevent_default();
                process_event(GameEvent::MouseUp(event));
            }
        });

    // Add event handler KeyDown
    document().add_event_listener({
        let process_event = process_event.clone();
        move |event: event::KeyDownEvent| {
            let process_event = &mut *process_event.lock().unwrap();
            process_event(GameEvent::KeyDown(event));
        }
    });

    // Add event handler BeforeUnload
    window().add_event_listener({
        let process_event = process_event.clone();
        move |_event: event::BeforeUnloadEvent| {
            let process_event = &mut *process_event.lock().unwrap();
            process_event(GameEvent::Exit);
        }
    });

    // Set up and start a timer if needed.
    if TICK_MS.is_some() {
        fn run_timer(process_event: Arc<Mutex<dyn FnMut(GameEvent)>>) {
            let process_event_clone = process_event.clone();
            stdweb::web::set_timeout(
                move || {
                    let process_event_fn = &mut *process_event_clone.lock().unwrap();
                    process_event_fn(GameEvent::Tick);
                    run_timer(process_event);
                },
                // XXX Because of the parent test, we know this
                // unwrap will succeed. But ugh.
                TICK_MS.unwrap(),
            );
        }
        run_timer(process_event);
    }

    // Start the event loop (which will never return)
    stdweb::event_loop();
}
