use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

const BORDER_X: f64 = 10.0;
const BORDER_Y: f64 = 10.0;
const OFFSET_Y: f64 = 20.0;
const BACKGROUND_COLOR: &str = &"#BBADA1";
pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new(attr_id: &str) -> Canvas {
        let canvas: CanvasElement = document()
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        canvas.set_width(600);
        canvas.set_height(600);
        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

        Canvas { canvas, ctx }
    }

    pub fn draw_tile(
        &self,
        x: usize,
        y: usize,
        size: usize,
        value: usize,
        foreground_color: &str,
        background_color: &str,
    ) {
        let scaled_width: f64 = f64::from(self.canvas.width()) / f64::from(size as u32);
        let scaled_height: f64 = f64::from(self.canvas.height()) / f64::from(size as u32);
        let scaled_x = f64::from(x as u32) * scaled_width;
        let scaled_y = f64::from(y as u32) * scaled_height;

        // draw rectangle
        self.ctx.set_fill_style_color(background_color);
        self.ctx.fill_rect(
            scaled_x + BORDER_X,
            scaled_y + BORDER_Y,
            scaled_width - 2.0 * BORDER_X,
            scaled_height - 2.0 * BORDER_Y,
        );

        // insert text
        self.ctx.set_fill_style_color(foreground_color);
        self.ctx.set_font("55px Sans-Serif");
        self.ctx.set_text_align(stdweb::web::TextAlign::Center);
        self.ctx.fill_text(
            &value.to_string(),
            scaled_x + 0.5 * scaled_width,
            scaled_y + 0.5 * scaled_height + OFFSET_Y,
            Some(scaled_width - 3.0 * BORDER_X),
        );
    }

    // pub fn draw_rect(&self, x: usize, y: usize, size: usize, color: &str) {
    //     let scaled_width: f64 = f64::from(self.canvas.width()) / f64::from(size as u32);
    //     let scaled_height: f64 = f64::from(self.canvas.height()) / f64::from(size as u32);
    //     self.ctx.set_fill_style_color(color);
    //     self.ctx.fill_rect(
    //         f64::from(x as u32) * scaled_width,
    //         f64::from(y as u32) * scaled_height,
    //         scaled_width,
    //         scaled_height,
    //     );
    // }

    pub fn clear_all(&self) {
        self.ctx.set_fill_style_color(BACKGROUND_COLOR);
        self.ctx.fill_rect(
            0.0,
            0.0,
            f64::from(self.canvas.width()),
            f64::from(self.canvas.height()),
        );
    }
}
