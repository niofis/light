use crate::light::vector::Vector;
use std::ops;
#[derive(Copy, Clone, Debug)]
pub struct Normal(pub f32, pub f32, pub f32); //x,y,z

impl ops::Mul<f32> for &Normal {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        let Normal(x, y, z) = self;
        Vector(x * rhs, y * rhs, z * rhs)
    }
}
