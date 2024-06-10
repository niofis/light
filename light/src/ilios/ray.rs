use super::{
    float::Float,
    geometry::{Normal, Point},
};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Normal,
    pub direction_reciprocal: [Float; 3],
    pub max_distance: Float,
    pub refraction_index: Float,
}

impl Ray {
    pub fn new(
        origin: Point,
        direction: Normal,
        max_distance: Float,
        refraction_index: Float,
    ) -> Ray {
        Ray {
            origin,
            direction,
            direction_reciprocal: [1.0 / direction.0, 1.0 / direction.1, 1.0 / direction.2],
            max_distance,
            refraction_index,
        }
    }

    pub fn point(&self, rhs: Float) -> Point {
        let Ray {
            origin,
            direction,
            direction_reciprocal: _,
            max_distance: _,
            refraction_index: _,
        } = self;
        origin + (direction * rhs)
    }
}
