use raqote::{DrawTarget, SolidSource, PathBuilder, Source, DrawOptions};

use crate::{CAR_HEIGHT, CAR_WIDTH, Point, Rect, SCALE, point2, Rotation, Car, MIRROR_HEIGHT, MIRROR_ANGLE, Map, WINDOW_WIDTH, WINDOW_HEIGHT};
use lazy_static::lazy_static;

const ROAD_WIDTH: f32 = CAR_HEIGHT * 1.5;
const PARKING_LENGTH: f32 = CAR_HEIGHT+0.7;
lazy_static! {
    static ref PARKING_WIDTH: f32 = CAR_WIDTH+MIRROR_HEIGHT*2.*f32::sin(MIRROR_ANGLE)+0.6;
}

pub struct BackParking {
    road: Rect,
    parking_space: Rect,
}

impl BackParking {
    pub fn new() -> Self {
        let mut road = Rect::new(point2(WINDOW_WIDTH/2., WINDOW_HEIGHT/2.), ROAD_WIDTH, WINDOW_HEIGHT);
        road.rotate(Rotation::new(std::f32::consts::PI/2., road.origin));
        BackParking {
            road,
            parking_space: Rect::new(
                point2(road.origin.x, road.origin.y-ROAD_WIDTH/2.-PARKING_LENGTH/2.),
                *PARKING_WIDTH, PARKING_LENGTH),
        }
    }
}

impl Map for BackParking {
    fn draw(&mut self, dt: &mut DrawTarget) {
        dt.clear(SolidSource::from_unpremultiplied_argb(0xff, 0x00, 0x00, 0x00));
        let color = SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff);
        self.road.draw(dt, color);
        self.parking_space.draw(dt, color);
    }

    fn car(&self) -> Car {
        Car::new(point2(WINDOW_WIDTH-CAR_HEIGHT, self.road.origin.y), std::f32::consts::PI/2.)
    }
}