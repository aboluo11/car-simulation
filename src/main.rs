use back_parking::BackParking;
use linear_algebra::{Matrix, Vector2D};
use minifb::{Window, WindowOptions, Key, KeyRepeat, MouseButton, Scale};
use parallel_parking::ParallelParking;
use raqote::{DrawTarget, SolidSource, Source, DrawOptions, PathBuilder, Image, ExtendMode, FilterMode, Transform, BlendMode, AntialiasMode};
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::{SystemSource};

use button::Button;

mod linear_algebra;
mod parallel_parking;
mod back_parking;
mod button;

const WINDOW_WIDTH: f32 = 800./SCALE;
const WINDOW_HEIGHT: f32 = 800./SCALE;
const CAR_WIDTH: f32 = 1.837;
const CAR_HEIGHT: f32 = 4.765;
const LOGO_WIDTH: f32 = 1.0;
const WHEEL_WIDTH: f32 = 0.215;
const WHEEL_HEIGHT: f32 = WHEEL_WIDTH*0.55*2.+1./39.37*17.;
const TURNING_RADIUS: f32 = 5.5;
const TURNING_COUNT: i32 = 4;
const SCALE: f32 = 30.;
const TRACK_WIDTH: f32 = 1.58;
const FRONT_SUSPENSION: f32 = 0.92;
const REAR_SUSPENSION: f32 = 1.05;
const MIRROR_WIDTH: f32 = 0.08;
const MIRROR_HEIGHT: f32 = 0.35;
const MIRROR_ANGLE: f32 = 70./180.*std::f32::consts::PI;
const MIRROR_ORIGIN_TO_FRONT: f32 = 1.55-MIRROR_WIDTH/2.;


fn real2pixel(p: Point) -> Point {
    point2(p.x * SCALE, (WINDOW_HEIGHT - p.y) * SCALE)
}

fn pixel2real(p: Point) -> Point {
    point2(p.x / SCALE, WINDOW_HEIGHT-p.y/SCALE)
}

fn new_rotation_matrix(angle: f32) -> Matrix<2, 2> {
    Matrix::new([
        [f32::cos(angle), -f32::sin(angle)],
        [f32::sin(angle), f32::cos(angle)],
    ])
}

#[derive(Clone, Copy)]
pub struct Point {
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

impl From<(f32, f32)> for Point {
    fn from(p: (f32, f32)) -> Self {
        point2(p.0, p.1)
    }
}

impl From<Point> for raqote::Point {
    fn from(p: Point) -> Self {
        raqote::Point::new(p.x, p.y)
    }
}

impl From<Point> for (f32, f32) {
    fn from(p: Point) -> Self {
        (p.x, p.y)
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

#[derive(Clone, Copy)]
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

    fn draw(&self, dt: &mut DrawTarget, color: SolidSource) {
        dt.fill(
            &self.path(),
            &Source::Solid(color),
            &DrawOptions {
                blend_mode: BlendMode::Src,
                alpha: 1.,
                antialias: AntialiasMode::Gray,
            }
        );
    }

    fn path(&self) -> raqote::Path {
        let mut pb = PathBuilder::new();
        let (x, y) = real2pixel(self.lt()).into();
        pb.move_to(x, y);
        let (x, y) = real2pixel(self.rt()).into();
        pb.line_to(x, y);
        let (x, y) = real2pixel(self.rb()).into();
        pb.line_to(x, y);
        let (x, y) = real2pixel(self.lb()).into();
        pb.line_to(x, y);
        pb.close();
        pb.finish()
    }

    fn rotate_self(&mut self, rotation_matrix: Matrix<2,2>) {
        self.rotation_matrix = rotation_matrix.multiply_matrix(self.rotation_matrix);
    }

    fn rotate(&mut self, rotation: Rotation) {
        self.origin = self.origin.rotate(rotation);
        self.rotate_self(rotation.rotation_matrix);
    }

    fn forward(&mut self, distance: f32, rotation_matrix: Matrix<2,2>) {
        self.origin = self.origin.forward(distance, rotation_matrix);
    }
}

struct Logo {
    data: Vec<u32>,
    outline: Rect,
}

impl Logo {
    fn new(path: &std::path::Path, origin: Point, width: f32) -> Self {
        let svg = usvg::Tree::from_data(&std::fs::read(path).unwrap(), &usvg::Options::default().to_ref()).unwrap();
        let (svg_ori_width, svg_ori_height) = (svg.svg_node().size.width(), svg.svg_node().size.height());
        let height = (svg_ori_height/svg_ori_width) as f32 * width;
        let mut pixmap = tiny_skia::Pixmap::new((width*SCALE) as u32, (height*SCALE) as u32).unwrap();
        resvg::render(&svg, usvg::FitTo::Width((width*SCALE) as u32), pixmap.as_mut()).unwrap();
        let mut data = vec![];
        for chunk in pixmap.data().chunks(4) {
            if let &[r, g, b, a] = chunk {
                data.push(u32::from_be_bytes([a, r, g, b]));
            }
        }
        Logo {
            data,
            outline: Rect::new(origin, width, height),
        }
    }

    fn draw(&self, dt: &mut DrawTarget) {
        let image = raqote::Image {
            width: (self.outline.width*SCALE) as i32,
            height: (self.outline.height*SCALE) as i32,
            data: self.data.as_slice()
        };
        let rot_inv = self.outline.rotation_matrix.inverse().unwrap();
        dt.fill(
            &self.outline.path(), 
            &Source::Image(
                image,
                ExtendMode::Pad,
                FilterMode::Bilinear,
                Transform::row_major(1./SCALE, 0., 0., -1./SCALE, 0., WINDOW_HEIGHT)
                    .post_transform(&Transform::create_translation(-self.outline.origin.x, -self.outline.origin.y))
                    .post_transform(&Transform::row_major(
                        rot_inv.inner[0][0],
                        rot_inv.inner[1][0],
                        rot_inv.inner[0][1],
                        rot_inv.inner[1][1],
                        self.outline.width/2.,
                        self.outline.height/2.,
                    )).post_transform(&Transform::row_major(
                        SCALE, 0., 0., -SCALE, 0., self.outline.height*SCALE
                    ))
            ), 
            &DrawOptions {
                blend_mode: BlendMode::SrcOver,
                alpha: 1.,
                antialias: AntialiasMode::Gray,
            }
        );
    }

    fn rotate_self(&mut self, rotation_matrix: Matrix<2,2>) {
        self.outline.rotate_self(rotation_matrix);
    }

    fn rotate(&mut self, rotation: Rotation) {
        self.outline.rotate(rotation);
    }

    fn forward(&mut self, distance: f32, rotation_matrix: Matrix<2,2>) {
        self.outline.forward(distance, rotation_matrix);
    }
}

struct Car {
    lt: Rect,
    rt: Rect,
    lb: Rect,
    rb: Rect,
    body: Rect,
    steer_angle: i32,
    logo: Logo,
    left_mirror: Rect,
    right_mirror: Rect,
}

impl Car {
    fn new(body_origin: Point, angle: f32) -> Car {
        let mut body = Rect::new(body_origin, CAR_WIDTH, CAR_HEIGHT);
        let mut lt = Rect::new(point2(body.origin.x-TRACK_WIDTH/2., CAR_HEIGHT/2.+body.origin.y-FRONT_SUSPENSION),
        WHEEL_WIDTH, WHEEL_HEIGHT);
        let mut rt = Rect::new(point2(body.origin.x+TRACK_WIDTH/2., CAR_HEIGHT/2.+body.origin.y-FRONT_SUSPENSION),
        WHEEL_WIDTH, WHEEL_HEIGHT);
        let mut lb = Rect::new(point2(body.origin.x-TRACK_WIDTH/2., -CAR_HEIGHT/2.+body.origin.y+REAR_SUSPENSION),
        WHEEL_WIDTH, WHEEL_HEIGHT);
        let mut rb = Rect::new(point2(body.origin.x+TRACK_WIDTH/2., -CAR_HEIGHT/2.+body.origin.y+REAR_SUSPENSION),
        WHEEL_WIDTH, WHEEL_HEIGHT);
        let mut logo = Logo::new(
            std::path::Path::new("res/tesla.svg"),
            point2(body_origin.x, body_origin.y+CAR_HEIGHT/2.-0.2),
            LOGO_WIDTH,
        );
        logo.outline.origin.y -= logo.outline.height/2.;
        let mut left_mirror = Rect::new(
            point2(
                body_origin.x-CAR_WIDTH/2.-MIRROR_HEIGHT/2.,
                body_origin.y+CAR_HEIGHT/2.-MIRROR_ORIGIN_TO_FRONT,
            ), MIRROR_WIDTH, MIRROR_HEIGHT);
        let mut right_mirror = Rect::new(
            point2(
                body_origin.x+CAR_WIDTH/2.+MIRROR_HEIGHT/2.,
                body_origin.y+CAR_HEIGHT/2.-MIRROR_ORIGIN_TO_FRONT,
            ), MIRROR_WIDTH, MIRROR_HEIGHT);
        left_mirror.rotate_self(new_rotation_matrix(std::f32::consts::PI/2.));
        right_mirror.rotate_self(new_rotation_matrix(std::f32::consts::PI/2.));
        left_mirror.rotate(Rotation::new(std::f32::consts::PI/2.-MIRROR_ANGLE, left_mirror.rb()));
        right_mirror.rotate(Rotation::new(-(std::f32::consts::PI/2.-MIRROR_ANGLE), right_mirror.rt()));
        let rotation = Rotation::new(angle, body_origin);
        body.rotate(rotation);
        lt.rotate(rotation);
        rt.rotate(rotation);
        lb.rotate(rotation);
        rb.rotate(rotation);
        logo.rotate(rotation);
        left_mirror.rotate(rotation);
        right_mirror.rotate(rotation);
        Car {
            lt, rt, lb, rb, body, steer_angle: 0, logo, left_mirror, right_mirror
        }
    }

    fn draw(&self, dt: &mut DrawTarget) {
        let body_color = SolidSource::from_unpremultiplied_argb(0xff, 24, 174, 219);
        let wheel_color = SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0);
        self.body.draw(dt, body_color);
        self.lt.draw(dt, wheel_color);
        self.rt.draw(dt, wheel_color);
        self.lb.draw(dt, wheel_color);
        self.rb.draw(dt, wheel_color);
        self.logo.draw(dt);
        self.left_mirror.draw(dt, body_color);
        self.right_mirror.draw(dt, body_color);
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
            self.logo.rotate(rotation);
            self.left_mirror.rotate(rotation);
            self.right_mirror.rotate(rotation);
        } else {
            let rotation_matrix = self.body.rotation_matrix;
            self.lt.forward(distance, rotation_matrix);
            self.rt.forward(distance, rotation_matrix);
            self.lb.forward(distance, rotation_matrix);
            self.rb.forward(distance, rotation_matrix);
            self.body.forward(distance, rotation_matrix);
            self.logo.forward(distance, rotation_matrix);
            self.left_mirror.forward(distance, rotation_matrix);
            self.right_mirror.forward(distance, rotation_matrix);
        }
    }

    fn L(&self) -> f32 {
        distance_of(self.lt.origin, self.lb.origin)
    }

    fn T(&self) -> f32 {
        distance_of(self.lb.origin, self.rb.origin)
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
            Some(
                (TURNING_COUNT as f32)*(f32::sqrt(TURNING_RADIUS*TURNING_RADIUS-self.L()*self.L())-self.T()/2.)
                    /(angle as f32)
            )
        }
    }

    fn left_steer(&mut self) {
        if self.steer_angle < TURNING_COUNT {
            self.steer_angle += 1;
            self.steer();
        }
    }

    fn right_steer(&mut self) {
        if self.steer_angle > -TURNING_COUNT {
            self.steer_angle -= 1;
            self.steer();
        }
    }
}

trait Map {
    fn draw(&mut self, dt: &mut DrawTarget);
    fn car(&self) -> Car;
}


fn main() {
    let font = font_kit::font::Font::from_path("C:\\Windows\\Fonts\\Deng.ttf", 0)
        .unwrap();
    let mut dt = DrawTarget::new((WINDOW_WIDTH*SCALE) as i32, (WINDOW_HEIGHT*SCALE) as i32);
    let mut map: Box<dyn Map> = Box::new(ParallelParking::new());
    let mut car = map.car();
    let mut window = Window::new("Car-Simulation", 
    (WINDOW_WIDTH*SCALE) as usize, (WINDOW_HEIGHT*SCALE) as usize, WindowOptions {
                                    ..WindowOptions::default()
                                }).unwrap();
    let size = window.get_size();
    let back_parking_button = Button::new(
        pixel2real((100., 50.).into()).into(), 100./SCALE, 50./SCALE, 
    &|| Box::new(BackParking::new()), "倒车入库");
    let parallel_parking_button = Button::new(
        pixel2real((100., 150.).into()).into(), 100./SCALE, 50./SCALE,
        &|| Box::new(ParallelParking::new()), "侧方停车");
    let buttons = vec![back_parking_button, parallel_parking_button];
    while window.is_open() {
        if window.get_mouse_down(MouseButton::Left) {
            let pixel_point: Point = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap().into();
            let point: Point = pixel2real(pixel_point).into();
            for button in buttons.iter() {
                if button.in_range(point) {
                    map = button.on_click();
                    car = map.car();
                    break;
                }
            }
        }
        map.draw(&mut dt);
        for button in buttons.iter() {
            button.draw(&mut dt, &font);
        }
        if window.is_key_pressed(Key::Up, KeyRepeat::Yes) {
            car.forward(0.3);
        } else if window.is_key_pressed(Key::Down, KeyRepeat::Yes) {
            car.forward(-0.3);
        } else if window.is_key_pressed(Key::Left, KeyRepeat::Yes) {
            car.left_steer();
        } else if window.is_key_pressed(Key::Right, KeyRepeat::Yes) {
            car.right_steer();
        }
        car.draw(&mut dt);
        window.update_with_buffer(dt.get_data(), size.0, size.1).unwrap();
    }
}
