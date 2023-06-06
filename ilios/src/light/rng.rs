use crate::float::Float;

const MAX: Float = 4294967295.0;

pub struct XorRng {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl XorRng {
    pub fn new() -> XorRng {
        XorRng {
            x: 123456789,
            y: 362436069,
            z: 521288629,
            w: 88675123,
        }
    }

    pub fn gen(&mut self) -> Float {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ (t ^ (t >> 8));
        (self.w as Float) / MAX
    }
}

pub trait Rng {
    fn gen(&mut self) -> Float;
}

impl Rng for XorRng {
    fn gen(&mut self) -> Float {
        self.gen()
    }
}
