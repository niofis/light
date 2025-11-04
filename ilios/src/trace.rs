use ilios_types::ray::Ray;

use super::geometry::PackedTriangles;

pub trait Trace {
    fn trace(&self, ray: &Ray) -> Option<Vec<&PackedTriangles>>;
}
