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

    pub fn intersect(&self, ray: &Ray) -> bool {
        let Ray {
            origin,
            direction,
            max_distance: _,
        } = ray;
        let BoundingBox { min, max } = self;
        let dxi: f32 = 1.0 / direction.0;
        let dyi: f32 = 1.0 / direction.1;
        let dzi: f32 = 1.0 / direction.2;
        let (sx, rsx) = if dxi < 0.0 { (1, 0) } else { (0, 1) };
        let (sy, rsy) = if dyi < 0.0 { (1, 0) } else { (0, 1) };
        let (sz, rsz) = if dzi < 0.0 { (1, 0) } else { (0, 1) };

        let params = [min, max];
        let tmin = (params[sx].0 - origin.0) * dxi;
        let tymax = (params[rsy].1 - origin.1) * dyi;
        let tmax = (params[rsx].0 - origin.0) * dxi;
        let tymin = (params[sy].1 - origin.1) * dyi;
        if tmin > tymax || tymin > tmax {
            return false;
        }
        let tmin = if tymin > tmin { tymin } else { tmin };
        let tmax = if tymax < tmax { tymax } else { tmax };
        let tzmin = (params[sz].2 - origin.2) * dzi;
        let tzmax = (params[rsz].2 - origin.2) * dzi;

        !(tmin > tzmax || tzmin > tmax)
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
