use crate::light::bounding_box::*;
use crate::light::primitive::*;

pub enum BVH {
    Empty,
    Node {
        primitives: Vec<Primitive>,
        bounding_box: BoundingBox,
        left: Box<BVH>,
        right: Box<BVH>,
    },
}
