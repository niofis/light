use crate::light::ray::*;
use crate::light::vector::*;

pub struct Camera {
    pub eye: Vector,
    pub left_top: Vector,
    pub delta_right: Vector,
    pub delta_down: Vector,
}

impl Camera {
    pub fn new(
        eye: Vector,
        left_top: Vector,
        left_bottom: Vector,
        right_top: Vector,
        width: f32,
        height: f32,
    ) -> Camera {
        let delta_right = (&right_top - &left_top) / width;
        let delta_down = (&left_bottom - &left_top) / height;

        Camera {
            eye,
            left_top,
            delta_right,
            delta_down,
        }
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        let Camera {
            left_top,
            delta_right,
            delta_down,
            eye,
        } = self;

        let origin = eye.clone();
        let point = left_top + &(delta_right * x) + (delta_down * y);
        let direction = &point - eye;

        Ray(origin, direction.unit())
    }
}
