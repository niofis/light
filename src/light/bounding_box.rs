use crate::light::vector::*;
use crate::light::ray::*;

pub struct BoundingBox {
    pub min: Vector,
    pub max: Vector,
}

impl BoundingBox {
    fn centroid(&self) -> Vector {
        (&self.min + &self.max) / 2.0
    }

    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        None
    }
}
