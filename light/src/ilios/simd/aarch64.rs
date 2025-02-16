use std::arch::aarch64::{
    float32x4_t, uint32x4_t, vaddq_f32, vaddvq_f32, vaddvq_u32, vandq_u32, vceqq_f32, vcgeq_f32,
    vcgtq_f32, vcleq_f32, vcltq_f32, vdivq_f32, vfmaq_f32, vgetq_lane_f32, vld1q_dup_f32,
    vld1q_lane_f32, vmulq_f32, vreinterpretq_f32_u32, vreinterpretq_u32_f32, vsubq_f32,
};

pub type F32x4 = float32x4_t;
pub type Mask = uint32x4_t;

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
            1 => vld1q_lane_f32::<1>(&value, vector),
            2 => vld1q_lane_f32::<2>(&value, vector),
            3 => vld1q_lane_f32::<3>(&value, vector),
            _ => vld1q_lane_f32::<0>(&value, vector),
        }
    }
}

pub fn get(vector: F32x4, lane: usize) -> f32 {
    let idx = lane % 4;
    unsafe {
        match idx {
            1 => vgetq_lane_f32::<1>(vector),
            2 => vgetq_lane_f32::<2>(vector),
            3 => vgetq_lane_f32::<3>(vector),
            _ => vgetq_lane_f32::<0>(vector),
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
    let res = zero();
    let res = set(res, a, 0);
    let res = set(res, b, 1);
    let res = set(res, c, 2);
    let res = set(res, d, 3);
    res
}

pub fn splat(a: f32) -> F32x4 {
    unsafe { vld1q_dup_f32(&a) }
}

pub fn add(a: F32x4, b: F32x4) -> F32x4 {
    unsafe { vaddq_f32(a, b) }
}

pub fn acc(a: F32x4) -> f32 {
    unsafe { vaddvq_f32(a) }
}

pub fn sub(a: F32x4, b: F32x4) -> F32x4 {
    unsafe { vsubq_f32(a, b) }
}

pub fn mul(a: F32x4, b: F32x4) -> F32x4 {
    unsafe { vmulq_f32(a, b) }
}

pub fn div(a: F32x4, b: F32x4) -> F32x4 {
    unsafe { vdivq_f32(a, b) }
}

pub fn mul_add(a: F32x4, b: F32x4, acc: F32x4) -> F32x4 {
    unsafe { vfmaq_f32(acc, a, b) }
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
    unsafe { vcgtq_f32(a, b) }
}

pub fn gte(a: F32x4, b: F32x4) -> Mask {
    unsafe { vcgeq_f32(a, b) }
}

pub fn lt(a: F32x4, b: F32x4) -> Mask {
    unsafe { vcltq_f32(a, b) }
}

pub fn lte(a: F32x4, b: F32x4) -> Mask {
    unsafe { vcleq_f32(a, b) }
}

pub fn eq(a: F32x4, b: F32x4) -> Mask {
    unsafe { vceqq_f32(a, b) }
}

pub fn and_mask(a: Mask, b: Mask) -> Mask {
    unsafe { vandq_u32(a, b) }
}

pub fn is_zero(a: Mask) -> bool {
    unsafe { vaddvq_u32(a) == 0 }
}

pub fn and_f32x4(a: F32x4, b: Mask) -> F32x4 {
    unsafe {
        let a_mask = vreinterpretq_u32_f32(a);
        let result = vandq_u32(a_mask, b);
        vreinterpretq_f32_u32(result)
    }
}
