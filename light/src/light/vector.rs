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
        let rec: f32 = 1.0 / rhs;
        Vector(x * rec, y * rec, z * rec)
    }
}

impl ops::Div<f32> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Vector {
        let Vector(x, y, z) = self;
        let rec: f32 = 1.0 / rhs;
        Vector(x * rec, y * rec, z * rec)
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
    pub fn norm(&self) -> f32 {
        self.dot(&self).sqrt()
    }
    pub fn unit(&self) -> Vector {
        self / self.norm()
    }
    pub fn cross(&self, rhs: &Vector) -> Vector {
        // let Vector(x1, y1, z1) = self;
        // let Vector(x2, y2, z2) = rhs;
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
}
