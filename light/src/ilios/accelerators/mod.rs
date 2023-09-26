use self::{
    bounding_volume_hierarchy::{BVHStats, Bvh},
    brute_force::BruteForce,
};
use super::{primitives::Primitive, ray::Ray, trace::Trace};
mod bounding_volume_hierarchy;
mod brute_force;

pub enum Accelerator {
    BruteForce,
    BoundingVolumeHierarchy,
}

#[derive(Clone, Debug)]
pub enum AcceleratorInstance {
    None,
    BruteForce(BruteForce),
    BoundingVolumeHierarchy(Bvh),
}

#[derive(Clone, Debug)]
pub enum AcceleratorStats {
    None,
    BoundingVolumeHierachy(BVHStats),
}

impl AcceleratorInstance {
    pub fn new_brute_force(primitives: &[Primitive]) -> AcceleratorInstance {
        let tracer = BruteForce::new(primitives);
        AcceleratorInstance::BruteForce(tracer)
    }
    pub fn new_bounding_volume_hierarchy(primitives: &[Primitive]) -> AcceleratorInstance {
        let tracer = Bvh::new(primitives);
        AcceleratorInstance::BoundingVolumeHierarchy(tracer)
    }
    pub fn stats(&self) -> AcceleratorStats {
        match self {
            AcceleratorInstance::BoundingVolumeHierarchy(tracer) => {
                AcceleratorStats::BoundingVolumeHierachy(tracer.stats())
            }
            _ => AcceleratorStats::None,
        }
    }
    pub fn trace(&self, ray: &Ray) -> Option<Vec<usize>> {
        match self {
            AcceleratorInstance::BruteForce(tracer) => tracer.trace(ray),
            AcceleratorInstance::BoundingVolumeHierarchy(tracer) => tracer.trace(ray),
            AcceleratorInstance::None => None,
        }
    }
}
