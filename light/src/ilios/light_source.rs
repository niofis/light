use crate::ilios::float::Float;
use crate::ilios::point::Point;

#[derive(Clone, Debug)]
pub enum LightSource {
    Point(Point, Float),
}
