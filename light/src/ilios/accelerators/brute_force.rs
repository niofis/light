use std::sync::Arc;

use crate::ilios::{
    geometry::{PackedTriangles, Triangle},
    ray::Ray,
    trace::Trace,
};

#[derive(Clone, Debug)]
pub struct BruteForce {
    primitives: Vec<Arc<Triangle>>,
    packed_triangles: PackedTriangles,
}

impl BruteForce {
    pub fn new(primitives: Vec<Triangle>) -> BruteForce {
        BruteForce {
            primitives: primitives.into_iter().map(Arc::new).collect(),
            packed_triangles: PackedTriangles::default(),
        }
    }
}

impl Trace for BruteForce {
    fn trace(&self, _ray: &Ray) -> Option<Vec<&PackedTriangles>> {
        // Some(self.primitives.iter().map(Arc::as_ref).collect())
        Some(vec![&self.packed_triangles])
    }
}
