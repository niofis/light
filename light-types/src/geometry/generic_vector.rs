use crate::float::Float;

#[derive(Debug, Clone, Copy)]
pub struct GVector4(pub Float, pub Float, pub Float, pub Float);

use std::ops;

impl ops::Add<GVector4> for GVector4 {
    type Output = GVector4;

    fn add(self, rhs: GVector4) -> Self::Output {
        GVector4(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl ops::Add<&GVector4> for &GVector4 {
    type Output = GVector4;

    fn add(self, rhs: &GVector4) -> Self::Output {
        GVector4(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl ops::Sub<GVector4> for GVector4 {
    type Output = GVector4;

    fn sub(self, rhs: GVector4) -> Self::Output {
        GVector4(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl ops::Sub<&GVector4> for &GVector4 {
    type Output = GVector4;

    fn sub(self, rhs: &GVector4) -> Self::Output {
        GVector4(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl ops::Mul<Float> for GVector4 {
    type Output = GVector4;

    fn mul(self, rhs: Float) -> Self::Output {
        GVector4(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl ops::Mul<Float> for &GVector4 {
    type Output = GVector4;

    fn mul(self, rhs: Float) -> Self::Output {
        GVector4(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl ops::Div<Float> for GVector4 {
    type Output = GVector4;

    fn div(self, rhs: Float) -> Self::Output {
        let inv: Float = 1.0 / rhs;
        GVector4(self.0 * inv, self.1 * inv, self.2 * inv, self.3 * inv)
    }
}

impl ops::Div<Float> for &GVector4 {
    type Output = GVector4;

    fn div(self, rhs: Float) -> Self::Output {
        let inv: Float = 1.0 / rhs;
        GVector4(self.0 * inv, self.1 * inv, self.2 * inv, self.3 * inv)
    }
}

impl ops::Index<usize> for GVector4 {
    type Output = Float;

    fn index(&self, rhs: usize) -> &Self::Output {
        match rhs {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => &self.3,
        }
    }
}

impl GVector4 {
    pub fn default() -> GVector4 {
        GVector4(0.0, 0.0, 0.0, 0.0)
    }
    pub fn clone(&self) -> GVector4 {
        GVector4(self.0, self.1, self.2, self.3)
    }
}
