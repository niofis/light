use crate::light::ray::*;
use crate::light::vector::*;
use std::f32::{MAX, MIN};
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: Vector,
    pub max: Vector,
    pub centroid: Vector,
}

impl ops::Index<usize> for BoundingBox {
    type Output = Vector;

    fn index(&self, rhs: usize) -> &Self::Output {
        match rhs {
            0 => &self.min,
            _ => &self.max,
        }
    }
}

impl BoundingBox {
    pub fn new(min: Vector, max: Vector) -> BoundingBox {
        BoundingBox {
            min,
            max,
            centroid: (min + max) / 2.0,
        }
    }
    pub fn default() -> BoundingBox {
        BoundingBox {
            min: Vector(MAX, MAX, MAX),
            max: Vector(MIN, MIN, MIN),
            centroid: Vector::default(),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        let Ray {
            origin,
            direction: _,
            direction_reciprocal,
            max_distance: _,
        } = ray;
        let [dxi, dyi, dzi] = *direction_reciprocal;
        let (sx, rsx) = if dxi < 0.0 { (1, 0) } else { (0, 1) };
        let (sy, rsy) = if dyi < 0.0 { (1, 0) } else { (0, 1) };
        let (sz, rsz) = if dzi < 0.0 { (1, 0) } else { (0, 1) };

        let tmin = (self[sx].0 - origin.0) * dxi;
        let tymax = (self[rsy].1 - origin.1) * dyi;
        if tmin > tymax {
            return false;
        }
        let tmax = (self[rsx].0 - origin.0) * dxi;
        let tymin = (self[sy].1 - origin.1) * dyi;
        if tymin > tmax {
            return false;
        }
        let tmin = if tymin > tmin { tymin } else { tmin };
        let tmax = if tymax < tmax { tymax } else { tmax };
        let tzmin = (self[sz].2 - origin.2) * dzi;
        let tzmax = (self[rsz].2 - origin.2) * dzi;

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
        let centroid = (self.min + self.max) / 2.0;
        BoundingBox { min, max, centroid }
    }
}
