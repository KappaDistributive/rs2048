use stdweb::traits::*;
use stdweb::web::{document, Element, INode};

pub struct Scoreboard {
    pub scoreboard: Element,
}

impl Scoreboard {
    pub fn new(attr_id: &str) -> Self {
        let scoreboard: Element = document().query_selector(attr_id).unwrap().unwrap();
        Scoreboard { scoreboard }
    }
}
