use crate::float::Float;

use super::geometry::Point;

#[derive(Clone, Debug)]
pub enum LightSource {
    Point(Point, Float),
}
