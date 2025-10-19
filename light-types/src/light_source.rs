use crate::{float::Float, geometry::Point};

#[derive(Clone, Debug)]
pub enum LightSource {
    Point(Point, Float),
}
