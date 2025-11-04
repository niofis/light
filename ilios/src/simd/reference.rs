pub type F32x4 = [f32; 4];
pub type Mask = [u32; 4];

pub fn set(vector: F32x4, value: f32, lane: usize) -> F32x4 {
    let idx = lane % 4;
    let mut result = vector;
    result[idx] = value;
    result
}

pub fn get(vector: F32x4, lane: usize) -> f32 {
    let idx = lane % 4;
    vector[idx]
}

pub fn default() -> F32x4 {
    [0.0, 0.0, 0.0, 0.0]
}

pub fn new(a: f32, b: f32, c: f32, d: f32) -> F32x4 {
    [a, b, c, d]
}

pub fn splat(a: f32) -> F32x4 {
    [a, a, a, a]
}

pub fn add(a: F32x4, b: F32x4) -> F32x4 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

pub fn acc(a: F32x4) -> f32 {
    a[0] + a[1] + a[2] + a[3]
}

pub fn sub(a: F32x4, b: F32x4) -> F32x4 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

pub fn mul(a: F32x4, b: F32x4) -> F32x4 {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

pub fn div(a: F32x4, b: F32x4) -> F32x4 {
    [a[0] / b[0], a[1] / b[1], a[2] / b[2], a[3] / b[3]]
}

pub fn mul_add(a: F32x4, b: F32x4, c: F32x4) -> F32x4 {
    [
        a[0] * b[0] + c[0],
        a[1] * b[1] + c[1],
        a[2] * b[2] + c[2],
        a[3] * b[3] + c[3],
    ]
}

pub fn cross(x1: F32x4, y1: F32x4, z1: F32x4, x2: F32x4, y2: F32x4, z2: F32x4) -> [F32x4; 3] {
    let x = sub(mul(y1, z2), mul(z1, y2));
    let y = sub(mul(z1, x2), mul(x1, z2));
    let z = sub(mul(x1, y2), mul(y1, x2));
    [x, y, z]
}

pub fn dot(x1: F32x4, y1: F32x4, z1: F32x4, x2: F32x4, y2: F32x4, z2: F32x4) -> F32x4 {
    let res = mul_add(x1, x2, F32x4::default());
    let res = mul_add(y1, y2, res);
    mul_add(z1, z2, res)
}

pub fn gt(a: F32x4, b: F32x4) -> Mask {
    [
        if a[0] > b[0] { u32::MAX } else { 0 },
        if a[1] > b[1] { u32::MAX } else { 0 },
        if a[2] > b[2] { u32::MAX } else { 0 },
        if a[3] > b[3] { u32::MAX } else { 0 },
    ]
}

pub fn gte(a: F32x4, b: F32x4) -> Mask {
    [
        if a[0] >= b[0] { u32::MAX } else { 0 },
        if a[1] >= b[1] { u32::MAX } else { 0 },
        if a[2] >= b[2] { u32::MAX } else { 0 },
        if a[3] >= b[3] { u32::MAX } else { 0 },
    ]
}

pub fn lt(a: F32x4, b: F32x4) -> Mask {
    [
        if a[0] < b[0] { u32::MAX } else { 0 },
        if a[1] < b[1] { u32::MAX } else { 0 },
        if a[2] < b[2] { u32::MAX } else { 0 },
        if a[3] < b[3] { u32::MAX } else { 0 },
    ]
}

pub fn lte(a: F32x4, b: F32x4) -> Mask {
    [
        if a[0] <= b[0] { u32::MAX } else { 0 },
        if a[1] <= b[1] { u32::MAX } else { 0 },
        if a[2] <= b[2] { u32::MAX } else { 0 },
        if a[3] <= b[3] { u32::MAX } else { 0 },
    ]
}

pub fn eq(a: F32x4, b: F32x4) -> Mask {
    [
        if a[0] == b[0] { u32::MAX } else { 0 },
        if a[1] == b[1] { u32::MAX } else { 0 },
        if a[2] == b[2] { u32::MAX } else { 0 },
        if a[3] == b[3] { u32::MAX } else { 0 },
    ]
}

pub fn and_mask(a: Mask, b: Mask) -> Mask {
    [a[0] & b[0], a[1] & b[1], a[2] & b[2], a[3] & b[3]]
}

pub fn is_zero(a: Mask) -> bool {
    (a[0] + a[1] + a[2] + a[3]) == 0
}

pub fn and_f32x4(a: F32x4, b: Mask) -> F32x4 {
    [
        f32::from_bits(a[0].to_bits() & b[0]),
        f32::from_bits(a[1].to_bits() & b[1]),
        f32::from_bits(a[2].to_bits() & b[2]),
        f32::from_bits(a[3].to_bits() & b[3]),
    ]
}
