use std::sync::Arc;

use crate::{
    float::EPSILON,
    ilios::{
        ray::Ray,
        simd::{self, F32x4},
    },
};

use super::Triangle;

#[derive(Clone, Default, Debug)]
pub struct PackedTriangles {
    pub triangles: Vec<Arc<Triangle>>,
    pub origin_x: F32x4,
    pub origin_y: F32x4,
    pub origin_z: F32x4,
    pub edge1_x: F32x4,
    pub edge1_y: F32x4,
    pub edge1_z: F32x4,
    pub edge2_x: F32x4,
    pub edge2_y: F32x4,
    pub edge2_z: F32x4,
}

impl PackedTriangles {
    pub fn intersect(&self, ray: &Ray) -> Option<F32x4> {
        let epsilon = simd::splat(EPSILON);
        let epsilon_p1 = simd::splat(1.0 + EPSILON);
        let mask = simd::splat(1.0);
        let zero = simd::splat(0.0);

        let ray_origin_x = simd::splat(ray.origin.0);
        let ray_origin_y = simd::splat(ray.origin.1);
        let ray_origin_z = simd::splat(ray.origin.2);

        let direction_x = simd::splat(ray.direction.0);
        let direction_y = simd::splat(ray.direction.1);
        let direction_z = simd::splat(ray.direction.2);

        //let pvec = direction.cross(edge2);
        let [pvec_x, pvec_y, pvec_z] = simd::cross(
            direction_x,
            direction_y,
            direction_z,
            self.edge2_x,
            self.edge2_y,
            self.edge2_z,
        );

        let det = simd::dot(
            self.edge1_x,
            self.edge1_y,
            self.edge1_z,
            pvec_x,
            pvec_y,
            pvec_z,
        );

        let inv_det = simd::div(simd::splat(1.0), det);
        let tvec_x = simd::sub(ray_origin_x, self.origin_x);
        let tvec_y = simd::sub(ray_origin_y, self.origin_y);
        let tvec_z = simd::sub(ray_origin_z, self.origin_z);
        let u = simd::mul(
            simd::dot(tvec_x, tvec_y, tvec_z, pvec_x, pvec_y, pvec_z),
            inv_det,
        );

        let lt_u = simd::gte(u, zero);
        let gt_u = simd::lte(u, epsilon_p1);

        let mask_u = simd::mul(lt_u, gt_u);

        /*
        if u < 0.0 || u > 1.0 + EPSILON {
            return None;
        }
        */

        let mask = simd::mul(mask, mask_u);

        if simd::acc(mask) == 0.0 {
            return None;
        }

        let [qvec_x, qvec_y, qvec_z] = simd::cross(
            tvec_x,
            tvec_y,
            tvec_z,
            self.edge1_x,
            self.edge1_y,
            self.edge1_z,
        );

        let v = simd::mul(
            simd::dot(
                direction_x,
                direction_y,
                direction_z,
                qvec_x,
                qvec_y,
                qvec_z,
            ),
            inv_det,
        );

        let lt_v = simd::gte(v, zero);
        let gt_v = simd::lte(simd::add(u, v), epsilon_p1);

        let mask_v = simd::mul(lt_v, gt_v);

        /*
        if v < 0.0 || (u + v) > 1.0 + EPSILON {
            //add EPSILON to offset small precision errors
            return None;
        }
        */

        let mask = simd::mul(mask, mask_v);

        if simd::acc(mask) == 0.0 {
            return None;
        }

        let t = simd::mul(
            simd::dot(
                self.edge2_x,
                self.edge2_y,
                self.edge2_z,
                qvec_x,
                qvec_y,
                qvec_z,
            ),
            inv_det,
        );

        let mask_t = simd::gt(t, epsilon);

        let mask = simd::mul(mask, mask_t);

        if simd::acc(mask) == 0.0 {
            return None;
        }

        Some(simd::mul(t, mask))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ilios::{
            geometry::{Normal, PackedTriangles},
            ray::Ray,
            simd::{self},
        },
        Point,
    };

    #[test]
    fn test_intersect() {
        /*
            Triangle { origin: Point(15.0, -6.0000024, -15.0), edge1: Vector(-30.0, 0.0, 0.0), edge2: Vector(0.0, 4.7683716e-6, 30.0), normal: Normal(0.0, 1.0, -1.5894572e-7), material: Diffuse(Color(1.0, 1.0, 1.0)), pt2: Point(-15.0, -6.0000024, -15.0), pt3: Point(15.0, -5.9999976, 15.0) }
            Ray { origin: Point(0.80026245, 4.7896767, -45.0), direction: Normal(0.077010185, -0.26081756, 0.9623116), direction_reciprocal: [12.985295, -3.8340976, 1.0391644], max_distance: inf, refraction_index: 1.0 }
            Some(41.368675)

        Ray { origin: Point(0.80026245, 4.7896767, -45.0), direction: Normal(0.077010185, -0.26081756, 0.9623116), direction_reciprocal: [12.985295, -3.8340976, 1.0391644], max_distance: inf, refraction_index: 1.0 }

        Some(41.368675)
        */

        // let v0 = Point::new(15.0, -6.0000024, -15.0);
        // let origin = Point::new(0.80026245, 4.7896767, -45.0);
        // let edge1 = Vector::new(-30.0, 0.0, 0.0);
        // let edge2 = Vector::new(0.0, 4.7683716e-6, 30.0);
        // let direction = Normal::new(0.077010185, -0.26081756, 0.9623116);
        // let pvec = direction.cross(&edge2);

        // let edge1 = Vector::new(-30.0, 0.0, 0.0);
        // let det = edge1.dot(&pvec);
        // println!("AKI2 det: {:?}", det);
        // let inv_det = 1.0 / det;
        // let tvec = &origin - &v0;
        // let u = tvec.dot(&pvec) * inv_det;
        // println!("AKI2 u: {:?}", u);

        // let qvec = tvec.cross(&edge1);
        // let v = direction.dot(&qvec) * inv_det;
        // println!("AKI2 v: {:?}", v);

        let pt = PackedTriangles {
            triangles: vec![],
            origin_x: simd::new(15.0, 15.0, 15.0, 0.0),
            origin_y: simd::new(-6.0000024, -6.0000024, -6.0000024, 0.0),
            origin_z: simd::new(-15.0, -15.0, -15.0, 0.0),
            edge1_x: simd::new(-30.0, -30.0, -30.0, -30.0),
            edge1_y: simd::new(0.0, 0.0, 0.0, 0.0),
            edge1_z: simd::new(0.0, 0.0, 0.0, 0.0),
            edge2_x: simd::new(0.0, 0.0, 0.0, 0.0),
            edge2_y: simd::new(4.7683716e-6, 4.7683716e-6, 4.7683716e-6, 4.7683716e-6),
            edge2_z: simd::new(30.0, 30.0, 30.0, 30.0),
        };
        let ray = Ray::new(
            Point(0.80026245, 4.7896767, -45.0),
            Normal(0.077010185, -0.26081756, 0.9623116),
            f32::INFINITY,
            1.0,
        );
        assert_eq!(
            pt.intersect(&ray),
            Some(simd::new(41.368675, 41.368675, 41.368675, 0.0))
        );
    }
}
