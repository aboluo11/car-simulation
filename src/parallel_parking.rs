use raqote::{DrawTarget, SolidSource};

use crate::{CAR_HEIGHT, CAR_WIDTH, Rect, point2, Car, Map, WINDOW_WIDTH, WINDOW_HEIGHT, MENU_WIDTH, View};

const ROAD_WIDTH: f32 = CAR_WIDTH * 3.;
const PARKING_LENGTH: f32 = 6.7;
const PARKING_WIDTH: f32 = 3.0;
const MAP_WIDTH: f32 = WINDOW_WIDTH - MENU_WIDTH;
const MAP_HEIGHT: f32 = WINDOW_HEIGHT;


pub struct ParallelParking {
    road: Rect,
    parking_space: Rect,
}

impl ParallelParking {
    pub fn new() -> Self {
        let color = SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff);
        let road = Rect::new(point2(MAP_WIDTH/2., MAP_HEIGHT/2.), ROAD_WIDTH, MAP_HEIGHT, Some(color));
        let parking_space = Rect::new(
            point2(road.origin.x+ROAD_WIDTH/2.+PARKING_WIDTH/2., road.origin.y),
            PARKING_WIDTH, PARKING_LENGTH, Some(color));
        ParallelParking {
            road,
            parking_space,
        }
    }
}

impl Map for ParallelParking {
    fn car(&self) -> Car {
        Car::new(point2(self.road.origin.x, CAR_HEIGHT), 0.)
    }
}

impl View for ParallelParking {
    fn relative_translation(&self) -> crate::linear_algebra::Vector2D {
        (MENU_WIDTH, 0.).into()
    }

    fn draw(&self, dt: &mut DrawTarget, translation: crate::linear_algebra::Vector2D) {
        let translation = self.relative_translation() + translation;
        dt.clear(SolidSource::from_unpremultiplied_argb(0xff, 0x00, 0x00, 0x00));
        self.road.draw(dt, translation);
        self.parking_space.draw(dt, translation);
    }
}