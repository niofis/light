use std::sync::Arc;

use crate::ilios::{geometry::Triangle, ray::Ray, trace::Trace};

#[derive(Clone, Debug)]
pub struct BruteForce {
    primitives: Vec<Arc<Triangle>>,
}

impl BruteForce {
    pub fn new(primitives: Vec<Triangle>) -> BruteForce {
        BruteForce {
            primitives: primitives.into_iter().map(Arc::new).collect(),
        }
    }
}

impl Trace for BruteForce {
    fn trace(&self, _ray: &Ray) -> Option<Vec<&Triangle>> {
        Some(self.primitives.iter().map(Arc::as_ref).collect())
    }
}
