mod board;
mod direction;
mod game;

use pancurses::{curs_set, endwin, half_delay, initscr, noecho, Input};
use crate::board::Board;
use crate::direction::Direction;
use crate::game::Game;

fn main() {
    let mut game: Game = Game::new();
    game.init();
    let window = initscr();
    window.printw(game.to_string());
    window.refresh();
    noecho();
    curs_set(0);
    loop {
        match window.getch() {
            Some(Input::Character('w')) => game.step(&Direction::Up),
            Some(Input::Character('s')) => game.step(&Direction::Down),
            Some(Input::Character('a')) => game.step(&Direction::Left),
            Some(Input::Character('d')) => game.step(&Direction::Right),
            None => {},
            Some(_) => break,
        }
        half_delay(200);
        window.clear();
        window.printw(game.to_string());
        window.refresh();
    }
}
