use crate::{
    float::Float,
    geometry::{Point, Vector},
    ray::Ray,
    transform::Transform,
};

#[derive(Clone, Default, Debug)]
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

#[derive(Clone, Debug)]
pub struct Camera {
    pub eye: Point,
    pub left_top: Point,
    pub left_bottom: Point,
    pub right_top: Point,
    delta_right: Vector,
    delta_down: Vector,
    width: Float,
    height: Float,
    pub coordinate_system: CoordinateSystem,
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            eye: Point::new(0.0, 0.0, -5.0),
            left_top: Point::new(-8.0, 4.5, 5.0),
            left_bottom: Point::new(-8.0, -4.5, 5.0),
            right_top: Point::new(8.0, 4.5, 5.0),
            delta_right: Vector::default(),
            delta_down: Vector::default(),
            width: 0.0,
            height: 0.0,
            coordinate_system: CoordinateSystem::default(),
        }
    }
}

impl Camera {
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

        let origin = left_top + (delta_right * x + delta_down * y);
        let direction = &origin - eye;

        Ray::new(origin, direction.unit(), Float::INFINITY, 1.0)
    }

    pub fn apply_transform(&mut self, transform: &Transform) {
        self.eye = transform.apply(&self.eye);
        self.left_top = transform.apply(&self.left_top);
        self.left_bottom = transform.apply(&self.left_bottom);
        self.right_top = transform.apply(&self.right_top);
        self.init(self.width, self.height);
    }
}
