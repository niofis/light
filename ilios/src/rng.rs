const MAX: Float = 4294967295.0;

use std::sync::atomic::{AtomicU32, Ordering};

use ilios_types::float::Float;

static SEED: AtomicU32 = AtomicU32::new(123456789);

pub struct XorRng {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl XorRng {
    pub fn new() -> XorRng {
        let mut rng = XorRng {
            x: SEED.load(Ordering::SeqCst),
            y: 362436069,
            z: 521288629,
            w: 88675123,
        };
        SEED.store(rng.gen_u32(), Ordering::SeqCst);
        rng
    }

    pub fn r#gen(&mut self) -> Float {
        self.gen_u32() as Float / MAX
    }

    pub fn gen_u32(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ (t ^ (t >> 8));
        self.w
    }
}

pub trait Rng {
    fn r#gen(&mut self) -> Float;
}

impl Rng for XorRng {
    fn r#gen(&mut self) -> Float {
        self.r#gen()
    }
}
