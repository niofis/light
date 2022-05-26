use crate::light::point::Point;
use crate::light::ray::Ray;
use crate::light::vector::Vector;
use crate::Transform;

pub struct Camera {
    pub eye: Point,
    pub left_top: Point,
    pub left_bottom: Point,
    pub right_top: Point,
    delta_right: Vector,
    delta_down: Vector,
    width: f32,
    height: f32,
    pub normal: Vector,
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
            normal: Vector::default(),
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
            normal: Vector::default(),
        }
    }

    pub fn init(&mut self, width: f32, height: f32) {
        let delta_right = (&self.right_top - &self.left_top) / width;
        let delta_down = (&self.left_bottom - &self.left_top) / height;
        self.delta_down = delta_down;
        self.delta_right = delta_right;
        self.width = width;
        self.height = height;
        let edge1 = &self.right_top - &self.left_top;
        let edge2 = &self.left_bottom - &self.left_top;
        self.normal = edge1.cross(&edge2).unit();
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        let Camera {
            eye,
            left_top,
            delta_right,
            delta_down,
            ..
        } = self;

        let origin = left_top + &(delta_right * x + delta_down * y);
        let direction = &origin - eye;

        Ray::new(origin, direction.unit(), f32::INFINITY)
    }

    pub fn apply_transform(&mut self, transform: &Transform) {
        self.eye = transform.apply(&self.eye);
        self.left_top = transform.apply(&self.left_top);
        self.left_bottom = transform.apply(&self.left_bottom);
        self.right_top = transform.apply(&self.right_top);
        self.init(self.width, self.height);
    }
}
