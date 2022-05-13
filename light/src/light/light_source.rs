use crate::light::point::Point;

pub enum LightSource {
    Point(Point, f32),
}
