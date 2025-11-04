use ilios_types::{float::Float, ray::Ray};

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Float>;
}
