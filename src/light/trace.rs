use crate::light::primitive::*;
use crate::light::ray::*;

pub trait Trace {
    fn trace(&self, ray: &Ray) -> Option<Vec<usize>>;
}
