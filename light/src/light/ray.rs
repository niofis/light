use crate::light::vector::*;

pub struct Ray(pub Vector, pub Vector); //origin, direction

impl Ray {
    pub fn new(origin: &Vector, direction: &Vector) -> Ray {
        let Vector(ox, oy, oz) = origin;
        let Vector(dx, dy, dz) = direction;
        Ray(Vector(*ox, *oy, *oz), Vector(*dx, *dy, *dz))
    }

    pub fn point(&self, rhs: f32) -> Vector {
        let Ray(origin, direction) = self;
        origin + &(direction * rhs)
    }
}
