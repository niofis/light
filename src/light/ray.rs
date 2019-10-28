use crate::light::vector::*;

#[derive(Clone, Copy, Debug)]
pub struct Ray(pub Vector, pub Vector); //origin, direction

impl Ray {
    pub fn point(self, rhs: f32) -> Vector {
        let Ray(origin, direction) = self;
        origin + (direction * rhs)
    }
}
