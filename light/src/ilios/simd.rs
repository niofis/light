pub type F32x4 = [f32; 4];

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

pub fn gt(a: F32x4, b: F32x4) -> F32x4 {
    [
        if a[0] > b[0] { 1.0 } else { 0.0 },
        if a[1] > b[1] { 1.0 } else { 0.0 },
        if a[2] > b[2] { 1.0 } else { 0.0 },
        if a[3] > b[3] { 1.0 } else { 0.0 },
    ]
}

pub fn gte(a: F32x4, b: F32x4) -> F32x4 {
    [
        if a[0] >= b[0] { 1.0 } else { 0.0 },
        if a[1] >= b[1] { 1.0 } else { 0.0 },
        if a[2] >= b[2] { 1.0 } else { 0.0 },
        if a[3] >= b[3] { 1.0 } else { 0.0 },
    ]
}

pub fn lt(a: F32x4, b: F32x4) -> F32x4 {
    [
        if a[0] < b[0] { 1.0 } else { 0.0 },
        if a[1] < b[1] { 1.0 } else { 0.0 },
        if a[2] < b[2] { 1.0 } else { 0.0 },
        if a[3] < b[3] { 1.0 } else { 0.0 },
    ]
}

pub fn lte(a: F32x4, b: F32x4) -> F32x4 {
    [
        if a[0] <= b[0] { 1.0 } else { 0.0 },
        if a[1] <= b[1] { 1.0 } else { 0.0 },
        if a[2] <= b[2] { 1.0 } else { 0.0 },
        if a[3] <= b[3] { 1.0 } else { 0.0 },
    ]
}

pub fn eq(a: F32x4, b: F32x4) -> F32x4 {
    [
        if a[0] == b[0] { 1.0 } else { 0.0 },
        if a[1] == b[1] { 1.0 } else { 0.0 },
        if a[2] == b[2] { 1.0 } else { 0.0 },
        if a[3] == b[3] { 1.0 } else { 0.0 },
    ]
}

#[cfg(test)]
mod tests {
    use crate::ilios::simd::{self, cross, dot};

    #[test]
    fn test_cross() {
        assert_eq!(
            cross(
                simd::splat(1.0),
                simd::splat(2.0),
                simd::splat(3.0),
                simd::splat(3.0),
                simd::splat(4.0),
                simd::splat(5.0),
            ),
            [simd::splat(-2.0), simd::splat(4.0), simd::splat(-2.0)]
        );
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            dot(
                simd::splat(1.0),
                simd::splat(2.0),
                simd::splat(3.0),
                simd::splat(3.0),
                simd::splat(4.0),
                simd::splat(5.0),
            ),
            simd::splat(26.0),
        );
    }
}
