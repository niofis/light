use crate::light::float::Float;
use crate::light::point::Point;

pub enum LightSource {
    Point(Point, Float),
}
