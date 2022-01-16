use crate::{Rect, Point, Map, real2pixel, point2, View, linear_algebra::Vector2D};

use font_kit::font::Font;
use raqote::{DrawTarget, SolidSource, DrawOptions, Source};

pub struct Button<'a> {
    outline: Rect,
    call_back: &'a dyn Fn() -> Box<dyn Map>,
    text: &'static str,
    font: &'a Font,
}

impl<'a> Button<'a> {
    pub fn new(origin: Point, width: f32, height: f32, 
        call_back: &'a dyn Fn() -> Box<dyn Map>, text: &'static str, font: &'a Font) -> Button<'a> {
        Button {
            outline: Rect::new(origin, width, height, 
                Some(SolidSource::from_unpremultiplied_argb(0xff, 102, 252, 3))),
            call_back,
            text,
            font,
        }
    }

    pub fn on_click(&self) -> Box<dyn Map> {
        (self.call_back)()
    }

    pub fn in_range(&self, point: Point) -> bool {
        point.x > self.outline.lt().x && point.x < self.outline.rb().x &&
        point.y < self.outline.lt().y && point.y > self.outline.rb().y
    }
}

impl View for Button<'_> {
    fn relative_translation(&self) -> Vector2D {
        (0., 0.).into()
    }

    fn draw(&self, dt: &mut DrawTarget, translation: Vector2D) {
        let translation = self.relative_translation() + translation;
        self.outline.draw(dt, translation);
        let lb = self.outline.lb();
        dt.draw_text(self.font, 27.0, self.text, 
            real2pixel(point2(lb.x, lb.y+0.5)+translation).into(), &Source::Solid(
            SolidSource{r:0, g:0, b:0xff, a:0xff}
        ), &DrawOptions::new())
    }
}