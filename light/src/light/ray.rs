use crate::light::point::Point;
use crate::light::vector::Vector;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
    pub direction_reciprocal: [f32; 3],
    pub max_distance: f32,
    pub refraction_index: f32,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector, max_distance: f32, refraction_index: f32) -> Ray {
        Ray {
            origin,
            direction,
            direction_reciprocal: [1.0 / direction.0, 1.0 / direction.1, 1.0 / direction.2],
            max_distance,
            refraction_index,
        }
    }

    pub fn point(&self, rhs: f32) -> Point {
        let Ray {
            origin,
            direction,
            direction_reciprocal: _,
            max_distance: _,
            refraction_index: _,
        } = self;
        origin + &(direction * rhs)
    }
}
