use super::{geometry::Triangle, ray::Ray};

pub trait Trace {
    fn trace(&self, ray: &Ray) -> Option<Vec<&Triangle>>;
}
