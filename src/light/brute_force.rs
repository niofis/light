use crate::light::primitive::*;
use crate::light::ray::*;
use crate::light::trace::*;

pub struct BruteForce {
    primitives: Vec<Primitive>,
}

impl BruteForce {
    pub fn new(primitives: Vec<Primitive>) -> BruteForce {
        BruteForce { primitives }
    }
}

impl Trace for BruteForce {
    fn trace(&self, ray: &Ray) -> Option<Vec<&Primitive>> {
        let borrows = self
            .primitives
            .iter()
            //.map(|p| p)
            .collect::<Vec<&Primitive>>();
        Some(borrows)
    }
}
