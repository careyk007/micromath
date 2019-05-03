//! Four quadrant arctangent approximation for a single-precision float
//! using method described at:
//!
//! <https://ieeexplore.ieee.org/document/6375931>

use super::abs::abs;
use core::f32::consts::PI;

/// Computes an `atan2` approximation in radians.
pub(super) fn atan2_approx(y: f32, x: f32) -> f32 {
    let n = atan2_norm_approx(y, x);
    PI / 2.0 * if n > 2.0 { n - 4.0 } else { n }
}

/// Approximates `atan2(y,x)` normalized to the `[0, 4)` range with a maximum
/// error of `0.1620` degrees.
pub(super) fn atan2_norm_approx(y: f32, x: f32) -> f32 {
    const SIGN_MASK: u32 = 0x8000_0000;
    const B: f32 = 0.596_227;

    // Extract sign bits from floating point values
    let ux_s = SIGN_MASK & x.to_bits();
    let uy_s = SIGN_MASK & y.to_bits();

    // Determine quadrant offset
    let q = ((!ux_s & uy_s) >> 29 | ux_s >> 30) as f32;

    // Calculate arctangent in the first quadrant
    let bxy_a = abs(B * x * y);
    let n = bxy_a + y * y;
    let atan_1q = n / (x * x + bxy_a + n);

    // Translate it to the proper quadrant
    let uatan_2q = (ux_s ^ uy_s) | atan_1q.to_bits();
    q + f32::from_bits(uatan_2q)
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::consts::PI;

    #[test]
    fn sanity_check() {
        let atan2_test_vectors = [
            (0.0, 1.0, 0.0),
            (0.0, -1.0, PI),
            (3.0, 2.0, (3.0f32 / 2.0).atan()),
            (2.0, -1.0, (2.0f32 / -1.0).atan() + PI),
            (-2.0, -1.0, (-2.0f32 / -1.0).atan() - PI),
        ];

        for (y, x, expected) in &atan2_test_vectors {
            let actual = atan2_approx(*y, *x);
            let delta = actual - expected;

            assert!(
                // 0.1620 degrees in radians
                delta <= 0.003,
                "delta {} too large: {} vs {}",
                delta,
                actual,
                expected
            );
        }
    }
}