use linear_algebra::{Matrix, Vector2D};
use minifb::{MouseMode, Window, WindowOptions, Key, KeyRepeat};
use raqote::{DrawTarget, SolidSource, Source, DrawOptions, PathBuilder, StrokeStyle};

mod linear_algebra;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const CAR_WIDTH: f32 = 75.;
const CAR_HEIGHT: f32 = 130.;
const WHEEL_WIDTH: f32 = 15.;
const WHEEL_HEIGHT: f32 = 25.;

fn new_rotation_matrix(angle: f32) -> Matrix<2, 2> {
    Matrix::new([
        [f32::cos(angle), -f32::sin(angle)],
        [f32::sin(angle), f32::cos(angle)],
    ])
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn rotate(&mut self, rotation: Rotation) {
        let p: Vector2D = self.to_vector();
        let o: Vector2D = rotation.origin.to_vector();
        let p_new = rotation.rotation_matrix.multiply_matrix(p-o) + o;
        self.x = p_new.inner[0][0];
        self.y = p_new.inner[1][0];
    }

    fn forward(&mut self, distance: f32, rotation_matrix: Matrix<2, 2>) {
        let mut tmp = point2(self.x, self.y+distance);
        tmp.rotate(Rotation {
            rotation_matrix,
            origin: *self,
        });
        self.x = tmp.x;
        self.y = tmp.y;
    }

    fn to_vector(&self) -> Vector2D {
        Vector2D::new_from_x_and_y(self.x, self.y)
    }
}

fn point2(x: f32, y: f32) -> Point {
    Point {x, y}
}

fn distance(x: Point, y: Point) -> f32 {
    let a = x.to_vector() - y.to_vector();
    (a * a).sum()
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
            rotation_matrix: Matrix { inner: [
                [1., 0.],
                [0., 1.]
            ] }
        }
    }

    fn lt(&self) -> Point {
        let mut p = point2(self.origin.x - self.width/2., self.origin.y - self.height/2.);
        p.rotate(Rotation { rotation_matrix: self.rotation_matrix, origin: self.origin });
        p
    }

    fn rt(&self) -> Point {
        let mut p = point2(self.origin.x + self.width/2., self.origin.y - self.height/2.);
        p.rotate(Rotation { rotation_matrix: self.rotation_matrix, origin: self.origin });
        p
    }

    fn lb(&self) -> Point {
        let mut p = point2(self.origin.x - self.width/2., self.origin.y + self.height/2.);
        p.rotate(Rotation { rotation_matrix: self.rotation_matrix, origin: self.origin });
        p
    }

    fn rb(&self) -> Point {
        let mut p = point2(self.origin.x + self.width/2., self.origin.y + self.height/2.);
        p.rotate(Rotation { rotation_matrix: self.rotation_matrix, origin: self.origin });
        p
    }

    fn draw(&self, pb: &mut PathBuilder) {
        pb.move_to(self.lt().x, self.lt().y);
        pb.line_to(self.rt().x, self.rt().y);
        pb.line_to(self.rb().x, self.rb().y);
        pb.line_to(self.lb().x, self.lb().y);
        pb.close();
    }

    fn rotate_self(&mut self, rotation_matrix: Matrix<2,2>) {
        self.rotation_matrix = rotation_matrix.multiply_matrix(self.rotation_matrix);
    }

    fn rotate(&mut self, rotation: Rotation) {
        self.origin.rotate(rotation);
        self.rotate_self(rotation.rotation_matrix);
    }

    fn forward(&mut self, distance: f32) {
        self.origin.forward(distance, self.rotation_matrix)
    }
}

struct Wheels {
    lt: Rect,
    rt: Rect,
    lb: Rect,
    rb: Rect,
}

impl Wheels {
    fn new(car: &Rect) -> Wheels {
        let lt = Rect::new(point2(-CAR_WIDTH/2.+car.origin.x+3.+WHEEL_WIDTH/2., -CAR_HEIGHT/4.+car.origin.y),
        CAR_WIDTH, CAR_HEIGHT);
        let rt = Rect::new(point2(car.origin.x+CAR_WIDTH/2.-3.-WHEEL_WIDTH/2., -CAR_HEIGHT/4.+car.origin.y),
        WHEEL_WIDTH, WHEEL_HEIGHT);
        let lb = Rect::new(point2(-CAR_WIDTH/2.+car.origin.x+3.+WHEEL_WIDTH/2., CAR_HEIGHT/4.+car.origin.y),
        WHEEL_WIDTH, WHEEL_HEIGHT);
        let rb = Rect::new(point2(car.origin.x+CAR_WIDTH/2.-3.-WHEEL_WIDTH/2., CAR_HEIGHT/4.+car.origin.y),
        WHEEL_WIDTH, WHEEL_HEIGHT);
        Wheels {
            lt, rt, lb, rb 
        }
    }

    fn draw(&self, pb: &mut PathBuilder) {
        self.lt.draw(pb);
        self.rt.draw(pb);
        self.lb.draw(pb);
        self.rb.draw(pb);
    }

    fn small_angle(&self, r: f32) -> f32 {
        f32::atan(self.L()/(r+self.T()/2.))
    }

    fn big_angle(&self, r: f32) -> f32 {
        f32::atan(self.L()/(r-self.T()/2.))
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

    fn top2_angle_matrix(&self, o: Point) -> (Matrix<2,2>, Matrix<2,2>) {
        let r = distance(self.back_origin(), o);
        if distance(self.lt.origin, o) < distance(self.rt.origin, o) {
            (self.big_angle_matrix(r), self.small_angle_matrix(r))
        } else {
            (-self.small_angle_matrix(r), -self.big_angle_matrix(r))
        }
    }

    fn top2_angle(&self, o: Point) -> (f32, f32) {
        let r = distance(self.back_origin(), o);
        if distance(self.lt.origin, o) < distance(self.rt.origin, o) {
            (self.big_angle(r), self.small_angle(r))
        } else {
            (-self.small_angle(r), -self.big_angle(r))
        }
    }

    fn steer(&mut self, o1: Point, o2: Point) {
        let r1 = distance(self.back_origin(), o1);
        let r2 = distance(self.back_origin(), o2);
        let (angle_lt1, angle_rt1) = self.top2_angle(o1);
        let (angle_lt2, angle_rt2) = self.top2_angle(o2);
        let angle_lt = angle_lt2 - angle_lt1;
        let angle_rt = angle_rt2 - angle_rt1;
        self.lt.rotate_self(Rotation::new(angle_lt, self.lt.origin));
        self.rt.rotate_self(Rotation::new(angle_rt, self.rt.origin));
    }

    fn forward(&mut self, distance: f32, o: Option<Point>) {
        if let Some(o) = o {

        } else {
            self.lt.forward(distance);
            self.rt.forward(distance);
            self.lb.forward(distance);
            self.rb.forward(distance);
        }
    }

    fn L(&self) -> f32 {
        self.lt.origin.y - self.lb.origin.y
    }

    fn T(&self) -> f32 {
        self.lb.origin.x - self.rb.origin.x
    }

    fn back_origin(&self) -> Point {
        point2((self.lb.origin.x+self.rb.origin.x)/2., (self.lb.origin.y+self.rb.origin.y)/2.)
    }
}

struct Car {
    body: Rect,
    wheels: Wheels,
}

impl Car {
    fn new(origin: Point) -> Car {
        let body = Rect::new(origin, CAR_WIDTH, CAR_HEIGHT);
        let wheel = Wheels::new(&body);
        Car {
            body,
            wheels: wheel,
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
        self.wheels.draw(&mut pb);
        dt.fill(
            &pb.finish(),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(0x7f, 0xff, 0, 0)),
            &DrawOptions::new()
        );
    }

    fn steer(&mut self, angle: f32) {

    }
}


fn main() {
    let mut window = Window::new("Raqote", WIDTH, HEIGHT, WindowOptions {
                                    ..WindowOptions::default()
                                }).unwrap();
    let size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);

    let mut car = Car::new(point2((WIDTH/2) as f32, (HEIGHT/2) as f32));
    loop {
        dt.clear(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff));
        if window.is_key_pressed(Key::Up, KeyRepeat::Yes) {
            // car.forward(10.);
        } else if window.is_key_pressed(Key::Down, KeyRepeat::Yes) {
            // car.origin.x += 10.;
        } else if window.is_key_pressed(Key::Left, KeyRepeat::Yes) {
            // angle -= 1.;
        } else if window.is_key_pressed(Key::Right, KeyRepeat::Yes) {
            // angle += 1.;
        }

        car.draw(&mut dt);
        window.update_with_buffer(dt.get_data(), size.0, size.1).unwrap();
    }
}
