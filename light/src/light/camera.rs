use crate::light::point::Point;
use crate::light::ray::Ray;
use crate::light::vector::Vector;

pub struct Camera {
    pub eye: Point,
    pub left_top: Point,
    pub left_bottom: Point,
    pub right_top: Point,
    delta_right: Vector,
    delta_down: Vector,
}

impl Camera {
    pub fn default() -> Camera {
        Camera {
            eye: Point::default(),
            left_top: Point::default(),
            left_bottom: Point::default(),
            right_top: Point::default(),
            delta_right: Vector::default(),
            delta_down: Vector::default(),
        }
    }

    pub fn new(eye: Point, left_top: Point, left_bottom: Point, right_top: Point) -> Camera {
        Camera {
            eye,
            left_top,
            left_bottom,
            right_top,
            delta_right: Vector::default(),
            delta_down: Vector::default(),
        }
    }

    pub fn init(&mut self, width: f32, height: f32) {
        let delta_right = (&self.right_top - &self.left_top) / width;
        let delta_down = (&self.left_bottom - &self.left_top) / height;
        self.delta_down = delta_down;
        self.delta_right = delta_right;
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        let Camera {
            eye,
            left_top,
            delta_right,
            delta_down,
            ..
        } = self;

        let origin = eye.clone();
        let point = left_top + &(delta_right * x + delta_down * y);
        let direction = &point - eye;

        Ray(origin, direction.unit())
    }
}
