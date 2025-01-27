use std::sync::Arc;

use crate::float::Float;

use super::geometry::Triangle;

#[derive(Debug)]
pub struct ClosestPrimitive<'a> {
    pub primitive: &'a Triangle,
    pub distance: Float,
}
