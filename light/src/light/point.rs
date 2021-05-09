use crate::light::vector::Vector;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Point(pub f32, pub f32, pub f32); //x,y,z

impl ops::Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        let Point(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        Point(x0 + x1, y0 + y1, z0 + z1)
    }
}

impl ops::Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        let Point(x0, y0, z0) = self;
        let Point(x1, y1, z1) = rhs;
        Point(x0 + x1, y0 + y1, z0 + z1)
    }
}

impl ops::Div<f32> for &Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Self::Output {
        let Point(x, y, z) = self;
        let inv: f32 = 1.0 / rhs;
        Point(x * inv, y * inv, z * inv)
    }
}

impl ops::Sub<&Point> for &Point {
    type Output = Vector;

    fn sub(self, rhs: &Point) -> Self::Output {
        let Point(x0, y0, z0) = self;
        let Point(x1, y1, z1) = rhs;
        Vector(x0 - x1, y0 - y1, z0 - z1)
    }
}

impl Point {
    pub fn default() -> Point {
        Point(0.0, 0.0, 0.0)
    }
}
