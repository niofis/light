use super::float::Float;
use crate::light::generic_vector::GVector4;
use crate::light::vector::Vector;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Point(pub Float, pub Float, pub Float); //x,y,z

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

impl From<Vector> for Point {
    fn from(pt: Vector) -> Self {
        Point(pt.0, pt.1, pt.2)
    }
}

impl ops::Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point::from(GVector4::from(self) + GVector4::from(rhs))
    }
}

impl ops::Add<Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::from(GVector4::from(self) + GVector4::from(rhs))
    }
}

impl ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::from(GVector4::from(self) + GVector4::from(rhs))
    }
}

impl ops::Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point::from(GVector4::from(self) + GVector4::from(rhs))
    }
}

impl ops::Div<Float> for &Point {
    type Output = Point;

    fn div(self, rhs: Float) -> Self::Output {
        Point::from(GVector4::from(self) / rhs)
    }
}

impl ops::Sub<&Point> for &Point {
    type Output = Vector;

    fn sub(self, rhs: &Point) -> Self::Output {
        Vector::from(GVector4::from(self) - GVector4::from(rhs))
    }
}

impl ops::Sub<&Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Point::from(GVector4::from(self) - GVector4::from(rhs))
    }
}

impl ops::Index<usize> for Point {
    type Output = Float;

    fn index(&self, rhs: usize) -> &Self::Output {
        match rhs {
            0 => &self.0,
            1 => &self.1,
            _ => &self.2,
        }
    }
}

impl Point {
    pub fn default() -> Point {
        Point(0.0, 0.0, 0.0)
    }
    pub fn distance(&self, rhs: &Point) -> Float {
        (rhs - self).norm()
    }
    pub fn distance_saquared(&self, rhs: &Point) -> Float {
        let dist = self.distance(rhs);
        dist * dist
    }
    pub fn min(&self, rhs: &Point) -> Point {
        Point(self.0.min(rhs.0), self.1.min(rhs.1), self.2.min(rhs.2))
    }
    pub fn max(&self, rhs: &Point) -> Point {
        Point(self.0.max(rhs.0), self.1.max(rhs.1), self.2.max(rhs.2))
    }
    pub fn floor(&self) -> Point {
        Point(self.0.floor(), self.1.floor(), self.2.floor())
    }
    pub fn ceil(&self) -> Point {
        Point(self.0.ceil(), self.1.ceil(), self.2.ceil())
    }
    pub fn abs(&self) -> Point {
        Point(self.0.abs(), self.1.abs(), self.2.abs())
    }
    pub fn permute(&self, x: usize, y: usize, z: usize) -> Point {
        Point(self[x], self[y], self[z])
    }
}
