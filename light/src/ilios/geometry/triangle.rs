use super::Normal;
use crate::float::{Float, EPSILON};
use crate::ilios::bounding_box::BoundingBox;
use crate::ilios::material::Material;
use crate::ilios::ray::Ray;
use crate::{Point, Vector};

#[derive(Debug, Clone)]
pub struct Triangle {
    pub origin: Point,
    pub edge1: Vector,
    pub edge2: Vector,
    pub normal: Normal,
    pub material: Material,
    pub pt2: Point,
    pub pt3: Point,
}

impl Triangle {
    pub fn new(pt1: Point, pt2: Point, pt3: Point, material: Material) -> Triangle {
        let edge1 = &pt2 - &pt1;
        let edge2 = &pt3 - &pt1;
        let normal = edge1.cross(&edge2).unit();

        Triangle {
            origin: pt1,
            edge1,
            edge2,
            normal,
            material,
            pt2,
            pt3,
        }
    }

    pub fn normal(&self) -> Normal {
        self.normal
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Float> {
        let Triangle {
            origin,
            edge1,
            edge2,
            ..
        } = self;
        triangle_intersect((origin, edge1, edge2), ray)
    }

    pub fn bounding_box(&self) -> BoundingBox {
        let Triangle {
            origin, pt2, pt3, ..
        } = self;

        BoundingBox::new(
            Point(
                origin.0.min(pt2.0).min(pt3.0),
                origin.1.min(pt2.1).min(pt3.1),
                origin.2.min(pt2.2).min(pt3.2),
            ),
            Point(
                origin.0.max(pt2.0).max(pt3.0),
                origin.1.max(pt2.1).max(pt3.1),
                origin.2.max(pt2.2).max(pt3.2),
            ),
        )
    }
}

fn triangle_intersect(triangle: (&Point, &Vector, &Vector), ray: &Ray) -> Option<Float> {
    let (v0, edge1, edge2) = triangle;
    let Ray {
        origin,
        direction,
        direction_reciprocal: _,
        max_distance: _,
        refraction_index: _,
    } = ray;
    let pvec = direction.cross(edge2);

    let det = edge1.dot(&pvec);
    //No culling version
    if det > -EPSILON && det < EPSILON {
        return None;
    }

    let inv_det = 1.0 / det;

    let tvec = origin - v0;

    let u = tvec.dot(&pvec) * inv_det;
    if u < 0.0 || u > 1.0 + EPSILON {
        return None;
    }

    let qvec = tvec.cross(edge1);

    let v = direction.dot(&qvec) * inv_det;
    if v < 0.0 || (u + v) > 1.0 + EPSILON {
        //add EPSILON to offset small precision errors
        return None;
    }

    let t = edge2.dot(&qvec) * inv_det;

    if t > EPSILON {
        return Some(t);
    }

    None
}
