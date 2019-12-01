use crate::light::primitive::*;
use crate::light::Trace::*;
use crate::light::ray::*;

pub struct BruteForce {
    primitives: Vec<&Primitive>
}

impl BruteForce {
    pub new(primitives: Vec<Primitive>) -> BruteForce {
        let prms = primitives.map(|p| &p).collect::<Vec<&Primitive>>();
        BruteForce(prms)
    }
}

impl Trace for BruteFormce {
    trace(&self, ray: &Ray) -> &[&Primitive]{
        Some(&self.prms[..]);
    }
