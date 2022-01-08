use crate::{Rect, Point, Map, real2pixel, point2};

use font_kit::font::Font;
use raqote::{DrawTarget, SolidSource, DrawOptions, Source};

pub struct Button<'a> {
    outline: Rect,
    call_back: &'a dyn Fn() -> Box<dyn Map>,
    text: &'static str,
}

impl<'a> Button<'a> {
    pub fn new(origin: Point, width: f32, height: f32, 
        call_back: &'a dyn Fn() -> Box<dyn Map>, text: &'static str) -> Button<'a> {
        Button {
            outline: Rect::new(origin, width, height),
            call_back,
            text,
        }
    }

    pub fn draw(&self, dt: &mut DrawTarget, font: &Font) {
        self.outline.draw(dt, SolidSource::from_unpremultiplied_argb(0xff, 104, 124, 166));
        let lb = self.outline.lb();
        dt.draw_text(font, 27.0, self.text, real2pixel(point2(lb.x, lb.y+0.5)).into(), &Source::Solid(
            SolidSource{r:0, g:0, b:0xff, a:0xff}
        ), &DrawOptions::new())
    }

    pub fn on_click(&self) -> Box<dyn Map> {
        (self.call_back)()
    }

    pub fn in_range(&self, point: Point) -> bool {
        point.x > self.outline.lt().x && point.x < self.outline.rb().x &&
        point.y < self.outline.lt().y && point.y > self.outline.rb().y
    }
}
