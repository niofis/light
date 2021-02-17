use super::{
    bounding_volume_hierarchy::BVH, brute_force::BruteForce, primitive::Primitive, ray::Ray,
    trace::Trace,
};

pub enum Accelerator {
    None,
    BruteForce(BruteForce),
    BoundingVolumeHierarchy(BVH),
}

impl Accelerator {
    pub fn new_brute_force(primitives: &Vec<Primitive>) -> Accelerator {
        let tracer = BruteForce::new(primitives);
        Accelerator::BruteForce(tracer)
    }
    pub fn new_bounding_volume_hierarchy(primitives: &Vec<Primitive>) -> Accelerator {
        let tracer = BVH::new(primitives);
        Accelerator::BoundingVolumeHierarchy(tracer)
    }
    pub fn trace(&self, ray: &Ray) -> Option<Vec<usize>> {
        match self {
            Accelerator::BruteForce(tracer) => tracer.trace(ray),
            Accelerator::BoundingVolumeHierarchy(tracer) => tracer.trace(ray),
            Accelerator::None => None,
        }
    }
}
