use ilios_types::{float::Float, geometry::Triangle};

#[derive(Debug)]
pub struct ClosestPrimitive<'a> {
    pub primitive: &'a Triangle,
    pub distance: Float,
}
