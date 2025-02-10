use super::{geometry::PackedTriangles, ray::Ray};

pub trait Trace {
    fn trace(&self, ray: &Ray) -> Option<Vec<&PackedTriangles>>;
}
