use super::{
    bounding_volume_hierarchy::{BVHStats, BVH},
    brute_force::BruteForce,
    primitive::Primitive,
    ray::Ray,
    trace::Trace,
};

pub enum Accelerator {
    BruteForce,
    BoundingVolumeHierarchy,
}

pub enum AcceleratorInstance {
    None,
    BruteForce(BruteForce),
    BoundingVolumeHierarchy(BVH),
}

#[derive(Debug)]
pub enum AcceleratorStats {
    None,
    BoundingVolumeHierachy(BVHStats),
}

impl AcceleratorInstance {
    pub fn new_brute_force(primitives: &Vec<Primitive>) -> AcceleratorInstance {
        let tracer = BruteForce::new(primitives);
        AcceleratorInstance::BruteForce(tracer)
    }
    pub fn new_bounding_volume_hierarchy(primitives: &Vec<Primitive>) -> AcceleratorInstance {
        let tracer = BVH::new(primitives);
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
