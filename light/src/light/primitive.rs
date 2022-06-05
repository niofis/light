use crate::light::bounding_box::BoundingBox;
use crate::light::material::Material;
use crate::light::point::Point;
use crate::light::ray::Ray;
use crate::light::vector::Vector;

#[derive(Debug, Clone)]
pub enum Primitive {
    Sphere {
        center: Point,
        radius: f32,
        material: Material,
    },
    Triangle {
        origin: Point,
        edge1: Vector,
        edge2: Vector,
        normal: Vector,
        material: Material,
        pt2: Point,
        pt3: Point,
        centroid: Point,
    },
}

impl Primitive {
    pub fn new_triangle(pt1: Point, pt2: Point, pt3: Point, material: Material) -> Primitive {
        let edge1 = &pt2 - &pt1;
        let edge2 = &pt3 - &pt1;
        let normal = edge1.cross(&edge2).unit();
        let centroid = &(&(&pt1 + &pt2) + &pt3) / 3.0;

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

    pub fn normal(&self, point: &Point) -> Vector {
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
            Primitive::Sphere { center, radius, .. } => BoundingBox::new(
                Point(center.0 - radius, center.1 - radius, center.2 - radius),
                Point(center.0 + radius, center.1 + radius, center.2 + radius),
            ),
            Primitive::Triangle {
                origin, pt2, pt3, ..
            } => BoundingBox::new(
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
            ),
        }
    }

    pub fn centroid(&self) -> Point {
        match self {
            Primitive::Sphere { center, .. } => *center,
            Primitive::Triangle { centroid, .. } => *centroid,
        }
    }
}

fn sphere_intersect(sphere: (&Point, &f32), ray: &Ray) -> Option<f32> {
    let (center, radius) = sphere;
    let Ray {
        origin,
        direction,
        direction_reciprocal: _,
        max_distance: _,
    } = ray;
    let oc = origin - center;
    let a = direction.dot(direction);
    let b = oc.dot(direction);
    let c = oc.dot(&oc) - radius * radius;
    let dis = b * b - a * c;
    let epsilon = 0.007;

    if dis > 0.0 {
        let e = dis.sqrt();

        let distance = (-b - e) / a;
        if distance > epsilon {
            return Some(distance);
        }

        let distance = (-b + e) / a;
        if distance > epsilon {
            return Some(distance);
        }
    }
    None
}

fn triangle_intersect(triangle: (&Point, &Vector, &Vector), ray: &Ray) -> Option<f32> {
    let (v0, edge1, edge2) = triangle;
    let Ray {
        origin,
        direction,
        direction_reciprocal: _,
        max_distance: _,
    } = ray;
    let pvec = direction.cross(edge2);
    let epsilon = 0.007;

    let det = edge1.dot(&pvec);
    //No culling version
    if det > -epsilon && det < epsilon {
        return None;
    }

    let inv_det = 1.0 / det;

    let tvec = origin - v0;

    let u = tvec.dot(&pvec) * inv_det;
    if u < 0.0 || u > 1.0 + epsilon {
        return None;
    }

    let qvec = tvec.cross(edge1);

    let v = direction.dot(&qvec) * inv_det;
    if v < 0.0 || (u + v) > 1.0 + epsilon {
        //add EPSILON to offset small precision errors
        return None;
    }

    let t = edge2.dot(&qvec) * inv_det;

    if t > epsilon {
        return Some(t);
    }

    None
}
