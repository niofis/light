use crate::light::ray::*;
use crate::light::vector::*;

pub struct Camera {
    pub eye: Vector,
    pub left_top: Vector,
    pub left_bottom: Vector,
    pub right_top: Vector,
    delta_right: Vector,
    delta_down: Vector,
}

impl Camera {
    pub fn default() -> Camera {
        Camera {
            eye: Vector::default(),
            left_top: Vector::default(),
            left_bottom: Vector::default(),
            right_top: Vector::default(),
            delta_right: Vector::default(),
            delta_down: Vector::default(),
        }
    }

    pub fn new(eye: Vector, left_top: Vector, left_bottom: Vector, right_top: Vector) -> Camera {
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
        let point = left_top + &(delta_right * x) + (delta_down * y);
        let direction = &point - eye;

        Ray(origin, direction.unit())
    }
}
