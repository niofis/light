use crate::light::normal::Normal;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vector(pub f32, pub f32, pub f32); //x,y,z

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        let Vector(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        Vector(x0 + x1, y0 + y1, z0 + z1)
    }
}

impl ops::Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        let Vector(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        Vector(x0 + x1, y0 + y1, z0 + z1)
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        let Vector(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        Vector(x0 - x1, y0 - y1, z0 - z1)
    }
}

impl ops::Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Self::Output {
        let Vector(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        Vector(x0 - x1, y0 - y1, z0 - z1)
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        let Vector(x, y, z) = self;
        Vector(x * rhs, y * rhs, z * rhs)
    }
}

impl ops::Mul<f32> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        let Vector(x, y, z) = self;
        Vector(x * rhs, y * rhs, z * rhs)
    }
}

impl ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        let Vector(x, y, z) = self;
        let inv: f32 = 1.0 / rhs;
        Vector(x * inv, y * inv, z * inv)
    }
}

impl ops::Div<f32> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        let Vector(x, y, z) = self;
        let inv: f32 = 1.0 / rhs;
        Vector(x * inv, y * inv, z * inv)
    }
}

impl ops::Index<usize> for Vector {
    type Output = f32;

    fn index(&self, rhs: usize) -> &Self::Output {
        if rhs == 0 {
            return &self.0;
        }
        if rhs == 1 {
            return &self.1;
        }
        return &self.2;
    }
}

impl Vector {
    pub fn default() -> Vector {
        Vector(0.0, 0.0, 0.0)
    }
    pub fn clone(&self) -> Vector {
        Vector(self.0, self.1, self.2)
    }
    pub fn dot(&self, rhs: &Vector) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
    pub fn abs_dot(&self, rhs: &Vector) -> f32 {
        self.dot(rhs).abs()
    }
    pub fn norm(&self) -> f32 {
        self.dot(&self).sqrt()
    }
    pub fn unit(&self) -> Vector {
        let Vector(x, y, z) = self / self.norm();
        Vector(x, y, z)
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
        Vector(x as f32, y as f32, z as f32)
    }
    pub fn min_component(&self) -> f32 {
        self.0.min(self.1.min(self.2))
    }
    pub fn max_component(&self) -> f32 {
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
        return 2;
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
}

impl From<&Normal> for Vector {
    fn from(normal: &Normal) -> Self {
        let Normal(x, y, z) = normal;
        Vector(*x, *y, *z)
    }
}
