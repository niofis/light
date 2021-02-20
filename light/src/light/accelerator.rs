use super::{
    bounding_volume_hierarchy::BVH, brute_force::BruteForce, primitive::Primitive, ray::Ray,
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

impl AcceleratorInstance {
    pub fn new_brute_force(primitives: &Vec<Primitive>) -> AcceleratorInstance {
        let tracer = BruteForce::new(primitives);
        AcceleratorInstance::BruteForce(tracer)
    }
    pub fn new_bounding_volume_hierarchy(primitives: &Vec<Primitive>) -> AcceleratorInstance {
        let tracer = BVH::new(primitives);
        AcceleratorInstance::BoundingVolumeHierarchy(tracer)
    }
    pub fn trace(&self, ray: &Ray) -> Option<Vec<usize>> {
        match self {
            AcceleratorInstance::BruteForce(tracer) => tracer.trace(ray),
            AcceleratorInstance::BoundingVolumeHierarchy(tracer) => tracer.trace(ray),
            AcceleratorInstance::None => None,
        }
    }
}
