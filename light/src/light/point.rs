use crate::light::generic_vector::GVector4;
use crate::light::vector::Vector;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Point(pub f32, pub f32, pub f32); //x,y,z

impl From<GVector4> for Point {
    fn from(gv: GVector4) -> Self {
        Point(gv.0, gv.1, gv.2)
    }
}

impl From<Point> for GVector4 {
    fn from(pt: Point) -> Self {
        GVector4(pt.0, pt.1, pt.2, 1.0)
    }
}

impl From<&Point> for GVector4 {
    fn from(pt: &Point) -> Self {
        GVector4(pt.0, pt.1, pt.2, 1.0)
    }
}

impl ops::Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point::from(GVector4::from(self) + GVector4::from(rhs))
    }
}

impl ops::Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point::from(GVector4::from(self) + GVector4::from(rhs))
    }
}

impl ops::Div<f32> for &Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Self::Output {
        Point::from(GVector4::from(self) * rhs)
    }
}

impl ops::Sub<&Point> for &Point {
    type Output = Vector;

    fn sub(self, rhs: &Point) -> Self::Output {
        Vector::from(GVector4::from(self) - GVector4::from(rhs))
    }
}

impl Point {
    pub fn default() -> Point {
        Point(0.0, 0.0, 0.0)
    }
}
