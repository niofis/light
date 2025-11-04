pub use bounding_volume_hierarchy::BvhBuildMethod;
use ilios_types::{geometry::Triangle, ray::Ray};

use self::{bounding_volume_hierarchy::BoundingVolumeHierarchy, brute_force::BruteForce};
use super::{geometry::PackedTriangles, trace::Trace};
mod bounding_volume_hierarchy;
mod brute_force;

#[derive(Clone, Debug)]
pub enum Accelerator {
    BruteForce,
    BoundingVolumeHierarchy,
}

#[derive(Clone, Debug)]
pub enum AcceleratorInstance {
    None,
    BruteForce(BruteForce),
    BoundingVolumeHierarchy(BoundingVolumeHierarchy),
}

impl AcceleratorInstance {
    pub fn new_brute_force(primitives: Vec<Triangle>) -> AcceleratorInstance {
        let tracer = BruteForce::new(primitives);
        AcceleratorInstance::BruteForce(tracer)
    }
    pub fn new_bounding_volume_hierarchy(
        build_method: BvhBuildMethod,
        primitives: Vec<Triangle>,
    ) -> AcceleratorInstance {
        let tracer = BoundingVolumeHierarchy::new(build_method, primitives);
        AcceleratorInstance::BoundingVolumeHierarchy(tracer)
    }
    pub fn trace(&self, ray: &Ray) -> Option<Vec<&PackedTriangles>> {
        match self {
            AcceleratorInstance::BruteForce(tracer) => tracer.trace(ray),
            AcceleratorInstance::BoundingVolumeHierarchy(tracer) => tracer.trace(ray),
            AcceleratorInstance::None => None,
        }
    }
}
