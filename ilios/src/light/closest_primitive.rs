use super::primitives::Primitive;
use crate::float::Float;

pub struct ClosestPrimitive<'a> {
    pub primitive: &'a Primitive,
    pub distance: Float,
}
