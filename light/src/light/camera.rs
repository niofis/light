use crate::light::point::Point;
use crate::light::ray::Ray;
use crate::light::vector::Vector;
use crate::Transform;

use super::float::Float;

#[derive(Default)]
pub struct CoordinateSystem {
    pub u: Vector,
    pub v: Vector,
    pub w: Vector,
}

impl CoordinateSystem {
    pub fn new(u: &Vector, v: &Vector) -> CoordinateSystem {
        let mut cs = CoordinateSystem::default();
        cs.u = u.unit().into();
        cs.v = v.unit().into();
        cs.w = cs.u.cross(&cs.v);
        cs
    }
}

pub struct Camera {
    pub eye: Point,
    pub left_top: Point,
    pub left_bottom: Point,
    pub right_top: Point,
    delta_right: Vector,
    delta_down: Vector,
    width: Float,
    height: Float,
    coordinate_system: CoordinateSystem,
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
            width: 0.0,
            height: 0.0,
            coordinate_system: CoordinateSystem::default(),
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
            width: 0.0,
            height: 0.0,
            coordinate_system: CoordinateSystem::default(),
        }
    }

    pub fn init(&mut self, width: Float, height: Float) {
        let delta_right = (&self.right_top - &self.left_top) / width;
        let delta_down = (&self.left_bottom - &self.left_top) / height;
        self.delta_down = delta_down;
        self.delta_right = delta_right;
        self.width = width;
        self.height = height;
        let edge1 = &self.right_top - &self.left_top;
        let edge2 = &self.left_bottom - &self.left_top;
        self.coordinate_system = CoordinateSystem::new(&edge1, &edge2);
    }

    pub fn get_ray(&self, x: Float, y: Float) -> Ray {
        let Camera {
            eye,
            left_top,
            delta_right,
            delta_down,
            ..
        } = self;

        let origin = left_top + &(delta_right * x + delta_down * y);
        let direction = &origin - eye;

        Ray::new(origin, direction.unit().into(), Float::INFINITY, 1.0)
    }

    pub fn rotate(&mut self, x: Float, y: Float, z: Float) {
        let transform = Transform::rotate(x, y, z);
        self.left_top = &transform.apply(&(&self.left_top - &self.eye).into()) + &self.eye;
        self.left_bottom = &transform.apply(&(&self.left_bottom - &self.eye).into()) + &self.eye;
        self.right_top = &transform.apply(&(&self.right_top - &self.eye).into()) + &self.eye;
        self.init(self.width, self.height);
    }

    pub fn translate(&mut self, x: Float, y: Float, z: Float) {
        let CoordinateSystem { u, v, w } = self.coordinate_system;
        let dw = w * -z;
        let du = u * -x;
        let dv = v * y;
        let transform = Transform::combine(&[
            Transform::translate(dw.0, dw.1, dw.2),
            Transform::translate(du.0, du.1, du.2),
            Transform::translate(dv.0, dv.1, dv.2),
        ]);

        self.eye = transform.apply(&self.eye);
        self.left_top = transform.apply(&self.left_top);
        self.left_bottom = transform.apply(&self.left_bottom);
        self.right_top = transform.apply(&self.right_top);
        self.init(self.width, self.height);
    }

    pub fn apply_transform(&mut self, transform: &Transform) {
        self.eye = transform.apply(&self.eye);
        self.left_top = transform.apply(&self.left_top);
        self.left_bottom = transform.apply(&self.left_bottom);
        self.right_top = transform.apply(&self.right_top);
        self.init(self.width, self.height);
    }
}
