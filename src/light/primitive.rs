use crate::light::bounding_box::*;
use crate::light::material::*;
use crate::light::ray::*;
use crate::light::vector::*;

#[derive(Debug)]
pub enum Primitive {
    Sphere {
        center: Vector,
        radius: f32,
        material: Material,
    },
    Triangle {
        origin: Vector,
        edge1: Vector,
        edge2: Vector,
        normal: Vector,
        material: Material,
        pt2: Vector,
        pt3: Vector,
        centroid: Vector,
    },
}

impl Primitive {
    pub fn new_triangle(pt1: Vector, pt2: Vector, pt3: Vector, material: Material) -> Primitive {
        let edge1 = &pt2 - &pt1;
        let edge2 = &pt3 - &pt1;
        let normal = edge1.cross(&edge2).unit();
        let centroid = (&(&pt1 + &pt2) + &pt3) / 3.0;

        Primitive::Triangle {
            origin: pt1,
            edge1,
            edge2,
            normal,
            material,
            pt2,
            pt3,
            centroid,
        }
    }

    pub fn normal(&self, point: &Vector) -> Vector {
        match self {
            Primitive::Sphere { center, .. } => ((point - center).unit()),
            Primitive::Triangle { normal, .. } => Vector(normal.0, normal.1, normal.2),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        match self {
            Primitive::Sphere { center, radius, .. } => sphere_intersect((center, radius), ray),
            Primitive::Triangle {
                origin,
                edge1,
                edge2,
                ..
            } => triangle_intersect((origin, edge1, edge2), ray),
        }
    }

    pub fn bounding_box(&self) -> BoundingBox {
        match self {
            Primitive::Sphere { center, radius, .. } => BoundingBox {
                min: Vector(center.0 - radius, center.1 - radius, center.2 - radius),
                max: Vector(center.0 + radius, center.1 + radius, center.2 + radius),
            },
            Primitive::Triangle {
                origin, pt2, pt3, ..
            } => BoundingBox {
                min: Vector(
                    origin.0.min(pt2.0).min(pt3.0),
                    origin.1.min(pt2.1).min(pt3.1),
                    origin.2.min(pt2.2).min(pt3.2),
                ),
                max: Vector(
                    origin.0.max(pt2.0).max(pt3.0),
                    origin.1.max(pt2.1).max(pt3.1),
                    origin.2.max(pt2.2).max(pt3.2),
                ),
            },
        }
    }

    pub fn centroid(&self) -> Vector {
        match self {
            Primitive::Sphere { center, .. } => center.clone(),
            Primitive::Triangle { centroid, .. } => centroid.clone(),
        }
    }
}

fn sphere_intersect(sphere: (&Vector, &f32), ray: &Ray) -> Option<f32> {
    let (center, radius) = sphere;
    let Ray(origin, direction) = ray;
    let oc = origin - &center;
    let a = direction.dot(direction);
    let b = oc.dot(direction);
    let c = oc.dot(&oc) - radius * radius;
    let dis = b * b - a * c;

    if dis > 0.0 {
        let e = dis.sqrt();

        let distance = (-b - e) / a;
        if distance > 0.007 {
            return Some(distance);
        }

        let distance = (-b + e) / a;
        if distance > 0.007 {
            return Some(distance);
        }
    }
    None
}

fn triangle_intersect(triangle: (&Vector, &Vector, &Vector), ray: &Ray) -> Option<f32> {
    let (v0, edge1, edge2) = triangle;
    let Ray(origin, direction) = ray;
    let pvec = direction.cross(edge2);

    let det = edge1.dot(&pvec);
    //No culling version
    if det > -0.007 && det < 0.007 {
        return None;
    }

    let inv_det = 1.0 / det;

    let tvec = origin - v0;

    let u = tvec.dot(&pvec) * inv_det;
    if u < 0.0 || u > 1.0 {
        return None;
    }

    let qvec = tvec.cross(edge1);

    let v = direction.dot(&qvec) * inv_det;
    if v < 0.0 || (u + v) > 1.007 {
        //add EPSILON to offset small precision errors
        return None;
    }

    let t = edge2.dot(&qvec) * inv_det;

    if t > 0.007 {
        return Some(t);
    }

    None
}
