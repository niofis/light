use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vector(pub f32, pub f32, pub f32); //x,y,z

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        let Vector(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        Vector(x0 + x1, y0 + y1, z0 + z1)
    }
}

impl ops::Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        let Vector(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        Vector(x0 + x1, y0 + y1, z0 + z1)
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        let Vector(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        Vector(x0 - x1, y0 - y1, z0 - z1)
    }
}

impl ops::Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Vector {
        let Vector(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        Vector(x0 - x1, y0 - y1, z0 - z1)
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Vector {
        let Vector(x, y, z) = self;
        Vector(x * rhs, y * rhs, z * rhs)
    }
}

impl ops::Mul<f32> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Vector {
        let Vector(x, y, z) = self;
        Vector(x * rhs, y * rhs, z * rhs)
    }
}

impl ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Vector {
        let Vector(x, y, z) = self;
        Vector(x / rhs, y / rhs, z / rhs)
    }
}

impl ops::Div<f32> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Vector {
        let Vector(x, y, z) = self;
        Vector(x / rhs, y / rhs, z / rhs)
    }
}

impl Vector {
    pub fn clone(&self) -> Vector {
        Vector(self.0, self.1, self.2)
    }
    pub fn dot(&self, rhs: &Vector) -> f32 {
        let Vector(x0, y0, z0) = self;
        let Vector(x1, y1, z1) = rhs;
        x0 * x1 + y0 * y1 + z0 * z1
    }
    pub fn norm(&self) -> f32 {
        self.dot(&self).sqrt()
    }
    pub fn unit(&self) -> Vector {
        self / self.norm()
    }
    pub fn cross(&self, rhs: &Vector) -> Vector {
        let Vector(x1, y1, z1) = self;
        let Vector(x2, y2, z2) = rhs;
        Vector(y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2)
    }
}
