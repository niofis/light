use crate::light::primitive::*;
use crate::light::ray::*;
use crate::light::trace::*;

pub struct BruteForce {
    primitives: Vec<usize>,
}

impl BruteForce {
    pub fn new(primitives: &[Primitive]) -> BruteForce {
        let primitives = (0..primitives.len()).collect();
        BruteForce { primitives }
    }
}

impl Trace for BruteForce {
    fn trace(&self, _ray: &Ray) -> Option<Vec<usize>> {
        Some(self.primitives.to_vec())
    }
}
