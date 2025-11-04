use std::arch::wasm32::{
    f32x4, f32x4_add, f32x4_div, f32x4_eq, f32x4_extract_lane, f32x4_ge, f32x4_gt, f32x4_le,
    f32x4_lt, f32x4_mul, f32x4_replace_lane, f32x4_splat, f32x4_sub, u32x4_extract_lane, v128,
    v128_and,
};

pub type F32x4 = v128;
pub type Mask = v128;

#[derive(Debug)]
pub struct ComparableF32x4(pub F32x4);

impl PartialEq for ComparableF32x4 {
    fn eq(&self, other: &Self) -> bool {
        get(self.0, 0) == get(other.0, 0)
            && get(self.0, 1) == get(other.0, 1)
            && get(self.0, 2) == get(other.0, 2)
            && get(self.0, 3) == get(other.0, 3)
    }
}

pub fn set(vector: F32x4, value: f32, lane: usize) -> F32x4 {
    let idx = lane % 4;
    unsafe {
        match idx {
            1 => f32x4_replace_lane::<1>(vector, value),
            2 => f32x4_replace_lane::<2>(vector, value),
            3 => f32x4_replace_lane::<3>(vector, value),
            _ => f32x4_replace_lane::<0>(vector, value),
        }
    }
}

pub fn get(vector: F32x4, lane: usize) -> f32 {
    let idx = lane % 4;
    unsafe {
        match idx {
            1 => f32x4_extract_lane::<1>(vector),
            2 => f32x4_extract_lane::<2>(vector),
            3 => f32x4_extract_lane::<3>(vector),
            _ => f32x4_extract_lane::<0>(vector),
        }
    }
}

pub fn default() -> F32x4 {
    zero()
}

fn zero() -> F32x4 {
    splat(0.0)
}

pub fn new(a: f32, b: f32, c: f32, d: f32) -> F32x4 {
    f32x4(a, b, c, d)
}

pub fn splat(a: f32) -> F32x4 {
    unsafe { f32x4_splat(a) }
}

pub fn add(a: F32x4, b: F32x4) -> F32x4 {
    unsafe { f32x4_add(a, b) }
}

pub fn acc(a: F32x4) -> f32 {
    unsafe {
        f32x4_extract_lane::<0>(a)
            + f32x4_extract_lane::<1>(a)
            + f32x4_extract_lane::<2>(a)
            + f32x4_extract_lane::<3>(a)
    }
}

pub fn sub(a: F32x4, b: F32x4) -> F32x4 {
    unsafe { f32x4_sub(a, b) }
}

pub fn mul(a: F32x4, b: F32x4) -> F32x4 {
    unsafe { f32x4_mul(a, b) }
}

pub fn div(a: F32x4, b: F32x4) -> F32x4 {
    unsafe { f32x4_div(a, b) }
}

pub fn mul_add(a: F32x4, b: F32x4, acc: F32x4) -> F32x4 {
    unsafe { f32x4_add(acc, f32x4_mul(a, b)) }
}

pub fn cross(x1: F32x4, y1: F32x4, z1: F32x4, x2: F32x4, y2: F32x4, z2: F32x4) -> [F32x4; 3] {
    let x = sub(mul(y1, z2), mul(z1, y2));
    let y = sub(mul(z1, x2), mul(x1, z2));
    let z = sub(mul(x1, y2), mul(y1, x2));
    [x, y, z]
}

pub fn dot(x1: F32x4, y1: F32x4, z1: F32x4, x2: F32x4, y2: F32x4, z2: F32x4) -> F32x4 {
    let res = mul_add(x1, x2, zero());
    let res = mul_add(y1, y2, res);
    mul_add(z1, z2, res)
}

pub fn gt(a: F32x4, b: F32x4) -> Mask {
    unsafe { f32x4_gt(a, b) }
}

pub fn gte(a: F32x4, b: F32x4) -> Mask {
    unsafe { f32x4_ge(a, b) }
}

pub fn lt(a: F32x4, b: F32x4) -> Mask {
    unsafe { f32x4_lt(a, b) }
}

pub fn lte(a: F32x4, b: F32x4) -> Mask {
    unsafe { f32x4_le(a, b) }
}

pub fn eq(a: F32x4, b: F32x4) -> Mask {
    unsafe { f32x4_eq(a, b) }
}

pub fn and_mask(a: Mask, b: Mask) -> Mask {
    unsafe { v128_and(a, b) }
}

pub fn is_zero(a: Mask) -> bool {
    unsafe {
        (u32x4_extract_lane::<0>(a)
            + u32x4_extract_lane::<1>(a)
            + u32x4_extract_lane::<2>(a)
            + u32x4_extract_lane::<3>(a))
            == 0
    }
}

pub fn and_f32x4(a: F32x4, b: Mask) -> F32x4 {
    unsafe { v128_and(a, b) }
}
