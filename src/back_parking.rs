use raqote::{DrawTarget, SolidSource};

use crate::{CAR_HEIGHT, CAR_WIDTH, Rect, point2, Rotation, Car, MIRROR_HEIGHT, MIRROR_ANGLE, Map, WINDOW_WIDTH, WINDOW_HEIGHT, View, MENU_WIDTH};
use lazy_static::lazy_static;

const ROAD_WIDTH: f32 = CAR_HEIGHT * 1.5;
const PARKING_LENGTH: f32 = CAR_HEIGHT+0.7;
lazy_static! {
    static ref PARKING_WIDTH: f32 = CAR_WIDTH+MIRROR_HEIGHT*2.*f32::sin(MIRROR_ANGLE)+0.6;
}
const MAP_WIDTH: f32 = WINDOW_WIDTH - MENU_WIDTH;
const MAP_HEIGHT: f32 = WINDOW_HEIGHT;

pub struct BackParking {
    road: Rect,
    parking_space: Rect,
}

impl BackParking {
    pub fn new() -> Self {
        let color = SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff);
        let mut road = Rect::new(point2(MAP_WIDTH/2., MAP_HEIGHT/2.), MAP_WIDTH, ROAD_WIDTH, Some(color));
        BackParking {
            road,
            parking_space: Rect::new(
                point2(road.origin.x, road.origin.y-ROAD_WIDTH/2.-PARKING_LENGTH/2.),
                *PARKING_WIDTH, PARKING_LENGTH, Some(color)),
        }
    }
}

impl Map for BackParking {
    fn car(&self) -> Car {
        Car::new(point2(MAP_WIDTH-CAR_HEIGHT, self.road.origin.y), std::f32::consts::PI/2.)
    }
}

impl View for BackParking {
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