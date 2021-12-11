use raqote::{DrawTarget, SolidSource, PathBuilder, Source, DrawOptions};

use crate::{CAR_HEIGHT, CAR_WIDTH, Rect, point2, Point};

const ROAD_WIDTH: f32 = CAR_WIDTH * 2.;
const PARKING_LENGTH: f32 = CAR_HEIGHT * 1.5;
const PARKING_WIDTH: f32 = CAR_WIDTH * 1.2;
pub const WINDOW_WIDTH: f32 = 800.;
pub const WINDOW_HEIGHT: f32 = 800.;


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
            dt: DrawTarget::new(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32),
        }
    }

    pub fn draw(&mut self) {
        self.dt.clear(SolidSource::from_unpremultiplied_argb(0xff, 0x00, 0x00, 0x00));
        let mut pb = PathBuilder::new();
        self.road.draw(&mut pb);
        self.parking_space.draw(&mut pb);
        self.dt.fill(
            &pb.finish(),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff)),
            &DrawOptions::new()
        );
    }

    pub fn car_start_origin(&self) -> Point {
        point2(self.road.origin.x, CAR_HEIGHT)
    }
}