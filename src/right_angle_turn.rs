use crate::{Rect, point2, WINDOW_HEIGHT, WINDOW_WIDTH, MENU_WIDTH, CAR_HEIGHT, Map, View, Car};

use raqote::{SolidSource, DrawTarget};

const MAP_WIDTH: f32 = WINDOW_WIDTH - MENU_WIDTH;
const MAP_HEIGHT: f32 = WINDOW_HEIGHT;
const ROAD_WIDTH: f32 = CAR_HEIGHT * 1.5;

pub struct RightAngleTurn {
    road_horizontal: Rect,
    road_vertical: Rect,
}

impl RightAngleTurn {
    pub fn new() -> Self {
        let color = SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff);
        let road_vertical = Rect::new(point2(MAP_WIDTH-ROAD_WIDTH/2.-0.3, MAP_HEIGHT/2.), 
            ROAD_WIDTH, MAP_HEIGHT-0.6, Some(color));
        let road_horizontal = Rect::new(point2(MAP_WIDTH/2., MAP_HEIGHT-ROAD_WIDTH/2.-0.3), 
            MAP_WIDTH-0.6, ROAD_WIDTH, Some(color));
        RightAngleTurn {
            road_horizontal,
            road_vertical,
        }
    }
}

impl Map for RightAngleTurn {
    fn car(&self) -> Car {
        Car::new(point2(self.road_vertical.origin.x, CAR_HEIGHT), 0.)
    }
}

impl View for RightAngleTurn {
    fn relative_translation(&self) -> crate::linear_algebra::Vector2D {
        (MENU_WIDTH, 0.).into()
    }

    fn draw(&self, dt: &mut DrawTarget, translation: crate::linear_algebra::Vector2D) {
        let translation = self.relative_translation() + translation;
        dt.clear(SolidSource::from_unpremultiplied_argb(0xff, 0x00, 0x00, 0x00));
        self.road_horizontal.draw(dt, translation);
        self.road_vertical.draw(dt, translation);
    }
}