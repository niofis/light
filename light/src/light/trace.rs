use super::ray::Ray;

pub trait Trace {
    fn trace(&self, ray: &Ray) -> Option<Vec<usize>>;
}
