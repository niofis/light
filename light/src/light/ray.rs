use crate::light::point::Point;
use crate::light::vector::Vector;

pub struct Ray(pub Point, pub Vector); //origin, direction

impl Ray {
    pub fn new(origin: &Point, direction: &Vector) -> Ray {
        let Point(ox, oy, oz) = origin;
        let Vector(dx, dy, dz) = direction;
        Ray(Point(*ox, *oy, *oz), Vector(*dx, *dy, *dz))
    }

    pub fn point(&self, rhs: f32) -> Point {
        let Ray(origin, direction) = self;
        origin + &(direction * rhs)
    }
}
