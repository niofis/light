#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub mod aarch64;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub use aarch64::*;

#[cfg(all(target_arch = "wasm32"))]
pub mod wasm32;
#[cfg(all(target_arch = "wasm32"))]
pub use wasm32::*;

#[cfg(all(not(target_arch = "aarch64"), not(target_arch = "wasm32")))]
pub mod reference;
#[cfg(all(not(target_arch = "aarch64"), not(target_arch = "wasm32")))]
pub use reference::*;

#[cfg(test)]
mod tests {
    use crate::ilios::simd::{self, cross, dot, ComparableF32x4};

    #[test]
    fn test_new() {
        assert_eq!(
            ComparableF32x4(simd::splat(1.0)),
            ComparableF32x4(simd::new(1.0, 1.0, 1.0, 1.0))
        )
    }

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
            )
            .map(|v| ComparableF32x4(v)),
            [
                ComparableF32x4(simd::splat(-2.0)),
                ComparableF32x4(simd::splat(4.0)),
                ComparableF32x4(simd::splat(-2.0))
            ]
        );
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            ComparableF32x4(dot(
                simd::splat(1.0),
                simd::splat(2.0),
                simd::splat(3.0),
                simd::splat(3.0),
                simd::splat(4.0),
                simd::splat(5.0),
            )),
            ComparableF32x4(simd::splat(26.0)),
        );
    }
}
