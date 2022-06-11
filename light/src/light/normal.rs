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
    type Output = Normal;

    fn add(self, rhs: Normal) -> Self::Output {
        Normal::from(GVector4::from(self) + GVector4::from(rhs))
    }
}

impl ops::Add<&Normal> for &Normal {
    type Output = Normal;

    fn add(self, rhs: &Normal) -> Self::Output {
        Normal::from(GVector4::from(self) + GVector4::from(rhs))
    }
}

impl ops::Sub<Normal> for Normal {
    type Output = Normal;

    fn sub(self, rhs: Normal) -> Self::Output {
        Normal::from(GVector4::from(self) - GVector4::from(rhs))
    }
}

impl ops::Sub<&Normal> for &Normal {
    type Output = Normal;

    fn sub(self, rhs: &Normal) -> Self::Output {
        Normal::from(GVector4::from(self) - GVector4::from(rhs))
    }
}

impl ops::Mul<Float> for &Normal {
    type Output = Normal;

    fn mul(self, rhs: Float) -> Self::Output {
        let Normal(x, y, z) = self;
        Normal(x * rhs, y * rhs, z * rhs)
    }
}

impl ops::Div<Float> for &Normal {
    type Output = Normal;

    fn div(self, rhs: Float) -> Self::Output {
        Normal::from(GVector4::from(self) / rhs)
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
        Normal(0.0, 0.0, 0.0)
    }
    pub fn dot(&self, rhs: &Normal) -> Float {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
    pub fn norm(&self) -> Float {
        self.dot(self).sqrt()
    }
    pub fn unit(&self) -> Normal {
        let Normal(x, y, z) = self / self.norm();
        Normal(x, y, z)
    }
    pub fn face_forward(&self, rhs: &Vector) -> Normal {
        let v1: Vector = self.into();
        if v1.dot(rhs) < 0.0 {
            -self
        } else {
            *self
        }
    }
}

impl From<&Vector> for Normal {
    fn from(normal: &Vector) -> Self {
        let Vector(x, y, z) = normal;
        Normal(*x, *y, *z)
    }
}
