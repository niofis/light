use std::sync::Arc;

use crate::ilios::{
    geometry::{PackedTriangles, Triangle},
    ray::Ray,
    simd,
    trace::Trace,
};

#[derive(Clone, Debug)]
pub struct BruteForce {
    packed_triangles: Vec<PackedTriangles>,
}

impl BruteForce {
    pub fn new(primitives: Vec<Triangle>) -> BruteForce {
        let mut packs = vec![];
        let mut pt = PackedTriangles::default();
        for triangle in primitives.iter() {
            if pt.triangles.len() == 4 {
                packs.push(pt);
                pt = PackedTriangles::default();
            }
            let i = pt.triangles.len();
            pt.triangles.push(Arc::new(triangle.clone()));
            pt.origin_x = simd::set(pt.origin_x, triangle.origin.0, i);
            pt.origin_y = simd::set(pt.origin_y, triangle.origin.1, i);
            pt.origin_z = simd::set(pt.origin_z, triangle.origin.2, i);
            pt.edge1_x = simd::set(pt.edge1_x, triangle.edge1.0, i);
            pt.edge1_y = simd::set(pt.edge1_y, triangle.edge1.1, i);
            pt.edge1_z = simd::set(pt.edge1_z, triangle.edge1.2, i);
            pt.edge2_x = simd::set(pt.edge2_x, triangle.edge2.0, i);
            pt.edge2_y = simd::set(pt.edge2_y, triangle.edge2.1, i);
            pt.edge2_z = simd::set(pt.edge2_z, triangle.edge2.2, i);
        }
        packs.push(pt);
        BruteForce {
            packed_triangles: packs,
        }
    }
}

impl Trace for BruteForce {
    fn trace(&self, _ray: &Ray) -> Option<Vec<&PackedTriangles>> {
        Some(self.packed_triangles.iter().collect())
    }
}
