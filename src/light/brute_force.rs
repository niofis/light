use crate::light::primitive::*;
use crate::light::trace::*;
use crate::light::ray::*;

pub struct BruteForce {
    primitives: Vec<Primitive>,
}

impl BruteForce {
    pub fn new(primitives: Vec<Primitive>) -> BruteForce {
        BruteForce {
            primitives,
        }
    }
}

impl Trace for BruteForce {
    fn trace(&self, ray: &Ray) -> Option<&[&Primitive]> {
        let borrows = self.primitives.map(|p| &p).collect::<Vec<&Primitive>>();
        Some(borrows[..]);
    }
}
