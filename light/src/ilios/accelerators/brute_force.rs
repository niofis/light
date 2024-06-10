use crate::ilios::{geometry::Triangle, ray::Ray, trace::Trace};

#[derive(Clone, Debug)]
pub struct BruteForce {
    primitives: Vec<usize>,
}

impl BruteForce {
    pub fn new(primitives: &[Triangle]) -> BruteForce {
        let primitives = (0..primitives.len()).collect();
        BruteForce { primitives }
    }
}

impl Trace for BruteForce {
    fn trace(&self, _ray: &Ray) -> Option<Vec<usize>> {
        Some(self.primitives.to_vec())
    }
}
