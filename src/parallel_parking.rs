use raqote::{DrawTarget, SolidSource, PathBuilder, Source, DrawOptions};

use crate::{CAR_HEIGHT, CAR_WIDTH, Point, Rect, SCALE, point2, Car};

const ROAD_WIDTH: f32 = CAR_WIDTH * 3.;
const PARKING_LENGTH: f32 = 6.7;
const PARKING_WIDTH: f32 = 2.6;
pub const WINDOW_WIDTH: f32 = 800./SCALE;
pub const WINDOW_HEIGHT: f32 = 800./SCALE;


pub struct ParallelParking {
    road: Rect,
    parking_space: Rect,
    pub dt: DrawTarget,
}

impl ParallelParking {
    pub fn new() -> Self {
        let road = Rect::new(point2(WINDOW_WIDTH/2., WINDOW_HEIGHT/2.), ROAD_WIDTH, WINDOW_HEIGHT);
        ParallelParking {
            road,
            parking_space: Rect::new(
                point2(road.origin.x+ROAD_WIDTH/2.+PARKING_WIDTH/2., road.origin.y),
                PARKING_WIDTH, PARKING_LENGTH),
            dt: DrawTarget::new((WINDOW_WIDTH*SCALE) as i32, (WINDOW_HEIGHT*SCALE) as i32),
        }
    }

    pub fn draw(&mut self) {
        self.dt.clear(SolidSource::from_unpremultiplied_argb(0xff, 0x00, 0x00, 0x00));
        let color = SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff);
        self.road.draw(&mut self.dt, color);
        self.parking_space.draw(&mut self.dt, color);
    }

    pub fn car(&self) -> Car {
        Car::new(point2(self.road.origin.x, CAR_HEIGHT), 0.)
    }
}