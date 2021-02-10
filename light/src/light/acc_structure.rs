use super::{
    bounding_volume_hierarchy::BVH, brute_force::BruteForce, primitive::Primitive, ray::Ray,
    trace::Trace,
};

pub enum AccStructure {
    BruteForce(BruteForce),
    BoundingVolumeHierarchy(BVH),
}

impl AccStructure {
    pub fn new_brute_force(primitives: &Vec<Primitive>) -> AccStructure {
        let tracer = BruteForce::new(primitives);
        AccStructure::BruteForce(tracer)
    }
    pub fn new_bounding_volume_hierarchy(primitives: &Vec<Primitive>) -> AccStructure {
        let tracer = BVH::new(primitives);
        AccStructure::BoundingVolumeHierarchy(tracer)
    }
    pub fn trace(&self, ray: &Ray) -> Option<Vec<usize>> {
        match self {
            AccStructure::BruteForce(tracer) => tracer.trace(ray),
            AccStructure::BoundingVolumeHierarchy(tracer) => tracer.trace(ray),
        }
    }
}
