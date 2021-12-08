use linear_algebra::{Matrix, Vector2D};
use minifb::{Window, WindowOptions, Key, KeyRepeat};
use raqote::{DrawTarget, SolidSource, Source, DrawOptions, PathBuilder};

mod linear_algebra;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const CAR_WIDTH: f32 = 75.;
const CAR_HEIGHT: f32 = 130.;
const WHEEL_WIDTH: f32 = 15.;
const WHEEL_HEIGHT: f32 = 25.;

fn coordinate_convert(y: f32) -> f32 {
    HEIGHT as f32 - y
}

fn new_rotation_matrix(angle: f32) -> Matrix<2, 2> {
    Matrix::new([
        [f32::cos(angle), -f32::sin(angle)],
        [f32::sin(angle), f32::cos(angle)],
    ])
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn rotate(&self, rotation: Rotation) -> Point {
        let p: Vector2D = self.to_vector();
        let o: Vector2D = rotation.origin.to_vector();
        let p_new = rotation.rotation_matrix.multiply_matrix(p-o) + o;
        point2(p_new.x(), p_new.y())
    }

    fn forward(&self, distance: f32, rotation_matrix: Matrix<2, 2>) -> Point {
        point2(self.x, self.y+distance).rotate(Rotation {
            rotation_matrix,
            origin: *self,
        })
    }

    fn to_vector(&self) -> Vector2D {
        Vector2D::new_from_x_and_y(self.x, self.y)
    }
}

fn point2(x: f32, y: f32) -> Point {
    Point {x, y}
}

fn distance_of(x: Point, y: Point) -> f32 {
    let a = x.to_vector() - y.to_vector();
    f32::sqrt((a * a).sum())
}

#[derive(Clone, Copy)]
struct Rotation {
    rotation_matrix: Matrix<2, 2>,
    origin: Point,
}

impl Rotation {
    fn new(angle: f32, origin: Point) -> Rotation {
        Rotation {
            rotation_matrix: new_rotation_matrix(angle),
            origin
        }
    }
}

struct Rect {
    origin: Point,
    width: f32,
    height: f32,
    rotation_matrix: Matrix<2,2>,
}

impl Rect {
    fn new(origin: Point, width: f32, height: f32) -> Rect {
        Rect {
            origin,
            width,
            height,
            rotation_matrix: Matrix::<2, 2>::eye(),
        }
    }

    fn lt(&self) -> Point {
        point2(self.origin.x - self.width/2., self.origin.y + self.height/2.)
            .rotate(Rotation { rotation_matrix: self.rotation_matrix, origin: self.origin })
    }

    fn rt(&self) -> Point {
        point2(self.origin.x + self.width/2., self.origin.y + self.height/2.)
            .rotate(Rotation { rotation_matrix: self.rotation_matrix, origin: self.origin })
    }

    fn lb(&self) -> Point {
        point2(self.origin.x - self.width/2., self.origin.y - self.height/2.)
            .rotate(Rotation { rotation_matrix: self.rotation_matrix, origin: self.origin })
    }

    fn rb(&self) -> Point {
        point2(self.origin.x + self.width/2., self.origin.y - self.height/2.)
            .rotate(Rotation { rotation_matrix: self.rotation_matrix, origin: self.origin })
    }

    fn draw(&self, pb: &mut PathBuilder) {
        pb.move_to(self.lt().x, coordinate_convert(self.lt().y));
        pb.line_to(self.rt().x, coordinate_convert(self.rt().y));
        pb.line_to(self.rb().x, coordinate_convert(self.rb().y));
        pb.line_to(self.lb().x, coordinate_convert(self.lb().y));
        pb.close();
    }

    fn rotate_self(&mut self, rotation_matrix: Matrix<2,2>) {
        self.rotation_matrix = rotation_matrix.multiply_matrix(self.rotation_matrix);
    }

    fn rotate(&mut self, rotation: Rotation) {
        self.origin = self.origin.rotate(rotation);
        self.rotate_self(rotation.rotation_matrix);
    }

    fn forward(&mut self, distance: f32) {
        self.origin = self.origin.forward(distance, self.rotation_matrix);
    }
}

struct Car {
    lt: Rect,
    rt: Rect,
    lb: Rect,
    rb: Rect,
    body: Rect,
    steer_angle: i32,
}

impl Car {
    fn new(body_origin: Point) -> Car {
        let body = Rect::new(body_origin, CAR_WIDTH, CAR_HEIGHT);
        let lt = Rect::new(point2(-CAR_WIDTH/2.+body.origin.x+3.+WHEEL_WIDTH/2., CAR_HEIGHT/4.+body.origin.y),
        WHEEL_WIDTH, WHEEL_HEIGHT);
        let rt = Rect::new(point2(body.origin.x+CAR_WIDTH/2.-3.-WHEEL_WIDTH/2., CAR_HEIGHT/4.+body.origin.y),
        WHEEL_WIDTH, WHEEL_HEIGHT);
        let lb = Rect::new(point2(-CAR_WIDTH/2.+body.origin.x+3.+WHEEL_WIDTH/2., -CAR_HEIGHT/4.+body.origin.y),
        WHEEL_WIDTH, WHEEL_HEIGHT);
        let rb = Rect::new(point2(body.origin.x+CAR_WIDTH/2.-3.-WHEEL_WIDTH/2., -CAR_HEIGHT/4.+body.origin.y),
        WHEEL_WIDTH, WHEEL_HEIGHT);
        Car {
            lt, rt, lb, rb, body, steer_angle: 0,
        }
    }

    fn draw(&self, dt: &mut DrawTarget) {
        let mut pb = PathBuilder::new();
        self.body.draw(&mut pb);
        dt.fill(
            &pb.finish(),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0xff, 0)),
            &DrawOptions::new()
        );

        let mut pb = PathBuilder::new();
        self.lt.draw(&mut pb);
        self.rt.draw(&mut pb);
        self.lb.draw(&mut pb);
        self.rb.draw(&mut pb);
        dt.fill(
            &pb.finish(),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(0x7f, 0xff, 0, 0)),
            &DrawOptions::new()
        );
    }

    fn angle_matrix(&self, r: f32) -> Matrix<2, 2> {
        let c = f32::sqrt(r*r + self.L()*self.L());
        Matrix { inner: [
            [r/c, -self.L()/c],
            [self.L()/c, r/c],
        ] }
    }

    fn small_angle_matrix(&self, r: f32) -> Matrix<2,2> {
        self.angle_matrix(r+self.T()/2.)
    }

    fn big_angle_matrix(&self, r: f32) -> Matrix<2,2> {
        self.angle_matrix(r-self.T()/2.)
    }

    fn top2_angle_matrix(&self, o: Option<Point>) -> (Matrix<2,2>, Matrix<2,2>) {
        match o {
            Some(o) => {
                let r = distance_of(self.back_origin(), o);
                if distance_of(self.lt.origin, o) < distance_of(self.rt.origin, o) {
                    (self.big_angle_matrix(r), self.small_angle_matrix(r))
                } else {
                    (self.small_angle_matrix(r).inverse().unwrap(), self.big_angle_matrix(r).inverse().unwrap())
                }
            },
            None => {
                (Matrix::<2, 2>::eye(), Matrix::<2, 2>::eye())
            }
        }
    }

    fn steer(&mut self) {
        let o_new = self.angle2origin(self.steer_angle);
        let (lt, rt) = self.top2_angle_matrix(o_new);
        self.lt.rotation_matrix = lt.multiply_matrix(self.body.rotation_matrix);
        self.rt.rotation_matrix = rt.multiply_matrix(self.body.rotation_matrix);
    }

    fn forward(&mut self, distance: f32) {
        let o = self.angle2origin(self.steer_angle);
        if let Some(o) = o {
            let angle = distance/distance_of(self.top_origin(), o) 
                * (if self.steer_angle > 0 {1.} else {-1.});
            let rotation = Rotation::new(angle, o);
            self.lt.rotate(rotation);
            self.rt.rotate(rotation);
            self.lb.rotate(rotation);
            self.rb.rotate(rotation);
            self.body.rotate(rotation);
        } else {
            self.lt.forward(distance);
            self.rt.forward(distance);
            self.lb.forward(distance);
            self.rb.forward(distance);
            self.body.forward(distance);
        }
    }

    fn L(&self) -> f32 {
        self.lt.origin.y - self.lb.origin.y
    }

    fn T(&self) -> f32 {
        self.rb.origin.x - self.lb.origin.x
    }

    fn back_origin(&self) -> Point {
        point2((self.lb.origin.x+self.rb.origin.x)/2., (self.lb.origin.y+self.rb.origin.y)/2.)
    }

    fn top_origin(&self) -> Point {
        point2((self.lt.origin.x+self.rt.origin.x)/2., (self.lt.origin.y+self.rt.origin.y)/2.)
    }

    fn angle2origin(&self, angle: i32) -> Option<Point> {
        match self.angle2r(angle) {
            Some(r) => {
                let back_origin = self.back_origin();
                let origin_before_trans = point2(back_origin.x-r, back_origin.y);
                Some(origin_before_trans.rotate(Rotation {
                    rotation_matrix: self.body.rotation_matrix,
                    origin: back_origin,
                }))
            },
            None => None
        }
    }

    fn angle2r(&self, angle: i32) -> Option<f32> {
        // angle>0: 向左转, r>0; angle<0: 向右转, r<0;
        if angle == 0 {
            None
        } else {
            Some(800./(angle as f32))
        }
    }

    fn left_steer(&mut self) {
        if self.steer_angle < 5 {
            self.steer_angle += 1;
            self.steer();
        }
    }

    fn right_steer(&mut self) {
        if self.steer_angle > -5 {
            self.steer_angle -= 1;
            self.steer();
        }
    }
}


fn main() {
    let mut window = Window::new("Car-Simulation", WIDTH, HEIGHT, WindowOptions {
                                    ..WindowOptions::default()
                                }).unwrap();
    let size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);

    let mut car = Car::new(point2((WIDTH/2) as f32, (HEIGHT/2) as f32));
    loop {
        dt.clear(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff));
        if window.is_key_pressed(Key::Up, KeyRepeat::Yes) {
            car.forward(10.);
        } else if window.is_key_pressed(Key::Down, KeyRepeat::Yes) {
            car.forward(-10.);
        } else if window.is_key_pressed(Key::Left, KeyRepeat::Yes) {
            car.left_steer();
        } else if window.is_key_pressed(Key::Right, KeyRepeat::Yes) {
            car.right_steer();
        }
        car.draw(&mut dt);
        window.update_with_buffer(dt.get_data(), size.0, size.1).unwrap();
    }
}
