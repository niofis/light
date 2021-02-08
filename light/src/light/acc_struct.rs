use super::{
    bounding_volume_hierarchy::BVH, brute_force::BruteForce, primitive::Primitive, ray::Ray,
    trace::Trace,
};

pub enum AccStruct {
    BruteForce(BruteForce),
    BoundingVolumeHierarchy(BVH),
}

impl AccStruct {
    pub fn new_brute_force(primitives: &Vec<Primitive>) -> AccStruct {
        let tracer = BruteForce::new(primitives);
        AccStruct::BruteForce(tracer)
    }
    pub fn new_bounding_volume_hierarchy(primitives: &Vec<Primitive>) -> AccStruct {
        let tracer = BVH::new(primitives);
        AccStruct::BoundingVolumeHierarchy(tracer)
    }
    pub fn trace(&self, ray: &Ray) -> Option<Vec<usize>> {
        match self {
            AccStruct::BruteForce(tracer) => tracer.trace(ray),
            AccStruct::BoundingVolumeHierarchy(tracer) => tracer.trace(ray),
        }
    }
}
