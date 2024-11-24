use super::{generic_vector::GVector4, Axis, Normal};
use crate::{float::Float, Point};
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vector(pub Float, pub Float, pub Float); //x,y,z

impl From<GVector4> for Vector {
    fn from(gv: GVector4) -> Self {
        Vector(gv.0, gv.1, gv.2)
    }
}

impl From<Vector> for GVector4 {
    fn from(vc: Vector) -> Self {
        GVector4(vc.0, vc.1, vc.2, 0.0)
    }
}

impl From<&Vector> for GVector4 {
    fn from(vc: &Vector) -> Self {
        GVector4(vc.0, vc.1, vc.2, 0.0)
    }
}

impl From<&Point> for Vector {
    fn from(pt: &Point) -> Self {
        Vector(pt.0, pt.1, pt.2)
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::from(GVector4::from(self) + GVector4::from(rhs))
    }
}

impl ops::Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        Vector::from(GVector4::from(self) + GVector4::from(rhs))
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::from(GVector4::from(self) - GVector4::from(rhs))
    }
}

impl ops::Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Vector::from(GVector4::from(self) - GVector4::from(rhs))
    }
}

impl ops::Mul<Float> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Float) -> Self::Output {
        Vector::from(GVector4::from(self) * rhs)
    }
}

impl ops::Mul<Float> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: Float) -> Self::Output {
        Vector::from(GVector4::from(self) * rhs)
    }
}

impl ops::Div<Float> for Vector {
    type Output = Vector;

    fn div(self, rhs: Float) -> Self::Output {
        Vector::from(GVector4::from(self) / rhs)
    }
}

impl ops::Div<Float> for &Vector {
    type Output = Vector;

    fn div(self, rhs: Float) -> Self::Output {
        Vector::from(GVector4::from(self) / rhs)
    }
}

impl ops::Index<usize> for Vector {
    type Output = Float;

    fn index(&self, rhs: usize) -> &Self::Output {
        match rhs {
            0 => &self.0,
            1 => &self.1,
            _ => &self.2,
        }
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector(-self.0, -self.1, -self.2)
    }
}

impl From<Normal> for Vector {
    fn from(normal: Normal) -> Self {
        Vector(normal.0, normal.1, normal.2)
    }
}

impl Default for Vector {
    fn default() -> Vector {
        Vector(0.0, 0.0, 0.0)
    }
}

impl Vector {
    pub fn new(x: Float, y: Float, z: Float) -> Vector {
        Vector(x, y, z)
    }
    pub fn dot(&self, rhs: &Vector) -> Float {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
    pub fn abs_dot(&self, rhs: &Vector) -> Float {
        self.dot(rhs).abs()
    }
    pub fn norm(&self) -> Float {
        self.dot(self).sqrt()
    }
    pub fn unit(&self) -> Normal {
        Normal::new(self.0, self.1, self.2)
    }
    pub fn cross(&self, rhs: &Vector) -> Vector {
        let x1 = self.0 as f64;
        let y1 = self.1 as f64;
        let z1 = self.2 as f64;
        let x2 = rhs.0 as f64;
        let y2 = rhs.1 as f64;
        let z2 = rhs.2 as f64;
        let x = y1 * z2 - z1 * y2;
        let y = z1 * x2 - x1 * z2;
        let z = x1 * y2 - y1 * x2;
        Vector(x as Float, y as Float, z as Float)
    }
    pub fn min_component(&self) -> Float {
        self.0.min(self.1.min(self.2))
    }
    pub fn max_component(&self) -> Float {
        self.0.max(self.1.max(self.2))
    }
    pub fn max_dimension(&self) -> usize {
        if self.0 > self.1 {
            if self.0 > self.2 {
                return 0;
            }
            return 2;
        } else if self.1 > self.2 {
            return 1;
        }
        2
    }
    pub fn min(&self, rhs: &Vector) -> Vector {
        Vector(self.0.min(rhs.0), self.1.min(rhs.1), self.2.min(rhs.2))
    }
    pub fn max(&self, rhs: &Vector) -> Vector {
        Vector(self.0.max(rhs.0), self.1.max(rhs.1), self.2.max(rhs.2))
    }
    pub fn permute(&self, x: usize, y: usize, z: usize) -> Vector {
        Vector(self[x], self[y], self[z])
    }
    pub fn coordinate_system(&self) -> (Vector, Vector) {
        let Vector(x, y, z) = self;
        let v2 = if x.abs() > y.abs() {
            Vector::new(-z, 0.0, *x) / (x * x + z * z)
        } else {
            Vector::new(0.0, *z, -y) / (y * y + z * z)
        };
        let v3 = self.cross(&v2);
        (v2, v3)
    }
    pub fn get_component(&self, axis: Axis) -> Float {
        match axis {
            Axis::X => self.0,
            Axis::Y => self.1,
            Axis::Z => self.2,
        }
    }
}

impl From<&Normal> for Vector {
    fn from(normal: &Normal) -> Self {
        let Normal(x, y, z) = normal;
        Vector(*x, *y, *z)
    }
}
