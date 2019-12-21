use crate::light::ray::*;
use crate::light::vector::*;
use std::f32::{MAX, MIN};

#[derive(Debug)]
pub struct BoundingBox {
    pub min: Vector,
    pub max: Vector,
}

impl BoundingBox {
    pub fn empty() -> BoundingBox {
        BoundingBox {
            min: Vector(MAX, MAX, MAX),
            max: Vector(MIN, MIN, MIN),
        }
    }
    pub fn centroid(&self) -> Vector {
        (&self.min + &self.max) / 2.0
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        let Ray(origin, direction) = ray;
        let BoundingBox { min, max } = self;
        let dxi: f32 = 1.0 / direction.0;
        let dyi: f32 = 1.0 / direction.1;
        let dzi: f32 = 1.0 / direction.2;
        let sign = vec![
            if dxi < 0.0 { 1 } else { 0 },
            if dyi < 0.0 { 1 } else { 0 },
            if dzi < 0.0 { 1 } else { 0 },
        ];
        let params = vec![min, max];
        let mut tmin = (params[sign[0]].0 - origin.0) * dxi;
        let mut tmax = (params[1 - sign[0]].0 - origin.0) * dxi;
        let tymin = (params[sign[1]].1 - origin.1) * dyi;
        let tymax = (params[1 - sign[1]].1 - origin.1) * dyi;
        if tmin > tymax || tymin > tmax {
            return false;
        }
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }
        let tzmin = (params[sign[2]].2 - origin.2) * dzi;
        let tzmax = (params[1 - sign[2]].2 - origin.2) * dzi;

        if tmin > tzmax || tzmin > tmax {
            return false;
        }
        true
    }

    pub fn combine(&self, rhs: &BoundingBox) -> BoundingBox {
        let min = Vector(
            self.min.0.min(rhs.min.0),
            self.min.1.min(rhs.min.1),
            self.min.2.min(rhs.min.2),
        );
        let max = Vector(
            self.max.0.max(rhs.max.0),
            self.max.1.max(rhs.max.1),
            self.max.2.max(rhs.max.2),
        );
        BoundingBox { min, max }
    }
}
