use crate::ilios::float::Float;
use crate::ilios::point::Point;

#[derive(Debug)]
pub enum LightSource {
    Point(Point, Float),
}
