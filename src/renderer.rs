use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

use crate::game::Game;

pub struct Renderer {
    pub canvas: CanvasElement,
    pub context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn new(id: &str) -> Self {
        let canvas: CanvasElement = document()
            .query_selector(id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        canvas.set_width(600);
        canvas.set_height(600);
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

        Renderer { canvas, context }
    }

    pub fn draw_rect(&self, x: f64, y: f64, width: f64, height: f64, color: &str) {
        self.context.set_fill_style_color(color);
        self.context.fill_rect(x, y, width, height);
    }

    pub fn clear_all(&self) {
        self.context.set_fill_style_color("white");
        self.context.fill_rect(
            0.0,
            0.0,
            f64::from(self.canvas.width() as u32),
            f64::from(self.canvas.height() as u32),
        );
    }
}
