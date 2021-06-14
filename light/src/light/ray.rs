use crate::light::point::Point;
use crate::light::vector::Vector;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
    pub max_distance: f32,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector, max_distance: f32) -> Ray {
        Ray {
            origin,
            direction,
            max_distance,
        }
    }

    pub fn point(&self, rhs: f32) -> Point {
        let Ray {
            origin,
            direction,
            max_distance: _,
        } = self;
        origin + &(direction * rhs)
    }
}
