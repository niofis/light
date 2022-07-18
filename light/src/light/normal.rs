use super::{float::Float, generic_vector::GVector4};
use crate::light::vector::Vector;
use std::ops;
#[derive(Copy, Clone, Debug)]
pub struct Normal(pub Float, pub Float, pub Float); //x,y,z

impl From<GVector4> for Normal {
    fn from(gv: GVector4) -> Self {
        Normal(gv.0, gv.1, gv.2)
    }
}

impl From<Normal> for GVector4 {
    fn from(vc: Normal) -> Self {
        GVector4(vc.0, vc.1, vc.2, 0.0)
    }
}

impl From<&Normal> for GVector4 {
    fn from(vc: &Normal) -> Self {
        GVector4(vc.0, vc.1, vc.2, 0.0)
    }
}

impl ops::Add<Normal> for Normal {
    type Output = Vector;

    fn add(self, rhs: Normal) -> Self::Output {
        Vector::from(GVector4::from(self) + GVector4::from(rhs))
    }
}

impl ops::Add<&Normal> for &Normal {
    type Output = Vector;

    fn add(self, rhs: &Normal) -> Self::Output {
        Vector::from(GVector4::from(self) + GVector4::from(rhs))
    }
}

impl ops::Sub<Normal> for Normal {
    type Output = Vector;

    fn sub(self, rhs: Normal) -> Self::Output {
        Vector::from(GVector4::from(self) - GVector4::from(rhs))
    }
}

impl ops::Sub<&Normal> for &Normal {
    type Output = Vector;

    fn sub(self, rhs: &Normal) -> Self::Output {
        Vector::from(GVector4::from(self) - GVector4::from(rhs))
    }
}

impl ops::Mul<Float> for &Normal {
    type Output = Vector;

    fn mul(self, rhs: Float) -> Self::Output {
        let Normal(x, y, z) = self;
        Vector(x * rhs, y * rhs, z * rhs)
    }
}

impl ops::Mul<Float> for Normal {
    type Output = Vector;

    fn mul(self, rhs: Float) -> Self::Output {
        let Normal(x, y, z) = self;
        Vector(x * rhs, y * rhs, z * rhs)
    }
}

impl ops::Div<Float> for &Normal {
    type Output = Vector;

    fn div(self, rhs: Float) -> Self::Output {
        Vector::from(GVector4::from(self) / rhs)
    }
}

impl ops::Neg for &Normal {
    type Output = Normal;
    fn neg(self) -> Self::Output {
        Normal(-self.0, -self.1, -self.2)
    }
}

impl Normal {
    pub fn new(x: Float, y: Float, z: Float) -> Normal {
        Normal(x, y, z)
    }
    pub fn default() -> Normal {
        Normal(1.0, 0.0, 0.0)
    }
    pub fn dot(&self, rhs: &Vector) -> Float {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
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
}

impl From<&Vector> for Normal {
    fn from(vector: &Vector) -> Self {
        vector.unit()
    }
}
