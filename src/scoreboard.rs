use stdweb::traits::*;
use stdweb::web::{document, Element};

pub struct Scoreboard {
    pub scoreboard: Element,
    pub best: Element,
}

impl Scoreboard {
    pub fn new(attr_id_scoreboard: &str, attr_id_best: &str) -> Self {
        let scoreboard: Element = document()
            .query_selector(attr_id_scoreboard)
            .unwrap()
            .unwrap();

        let best: Element = document().query_selector(attr_id_best).unwrap().unwrap();

        Scoreboard { scoreboard, best }
    }
}
