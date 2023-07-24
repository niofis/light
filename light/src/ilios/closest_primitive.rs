use super::primitives::Primitive;
use crate::float::Float;

#[derive(Debug)]
pub struct ClosestPrimitive<'a> {
    pub primitive: &'a Primitive,
    pub distance: Float,
}
