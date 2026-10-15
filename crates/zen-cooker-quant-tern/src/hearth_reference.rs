//! Pure `i32` reference for the ternary hearth Metal kernel.
//!
//! The Metal kernel (`kernels/ternary_hearth.metal`) accumulates
//! `y[o] = (Σ_i w[o,i] · x[i]) · γ` with branch-free sign selection. This
//! module is the same math in pure Rust — `i32` accumulation, one `f32`
//! scale at the end — and pins the canonical hearth problem as golden
//! constants, so the Swift parity runner and this test compare against the
//! same numbers.
//!
//! Canonical problem (the "hearth vector"): `n_in = 8`, `n_out = 2`,
//! weights and inputs fixed below, `γ = 0.5`.

/// The canonical hearth weight matrix, row-major `2 × 8`.
pub const HEARTH_W: [i8; 16] = [1, 0, -1, 1, 0, 1, -1, 1, -1, 1, 1, 0, -1, 1, 0, -1];

/// The canonical hearth input vector, `8 × 1`.
pub const HEARTH_X: [i16; 8] = [3, 7, 2, 9, 1, 4, 6, 5];

/// The canonical hearth scale (`γ = mean|W|` style, pinned).
pub const HEARTH_GAMMA: f32 = 0.5;

/// Golden output of the canonical hearth problem, computed once by this
/// reference and re-verified by the Swift Metal parity runner.
pub const HEARTH_GOLDEN: [f32; 2] = [6.5, 2.0];

/// Branch-free-sign ternary matvec, pure `i32` accumulation.
pub fn ternary_hearth_i32(w: &[i8], x: &[i16], n_out: usize, n_in: usize, gamma: f32) -> Vec<f32> {
    let mut y = vec![0.0f32; n_out];
    for o in 0..n_out {
        let mut acc: i32 = 0;
        for i in 0..n_in {
            let wi = w[o * n_in + i] as i32;
            let xi = x[i] as i32;
            // The directive's exact select form; predicated on GPU.
            acc += if wi == 1 {
                xi
            } else if wi == -1 {
                -xi
            } else {
                0
            };
        }
        y[o] = acc as f32 * gamma;
    }
    y
}

/// The canonical hearth run — the numbers both the Rust test and the Swift
/// runner must reproduce within `max_abs <= 1e-5`.
pub fn hearth_canonical() -> Vec<f32> {
    ternary_hearth_i32(&HEARTH_W, &HEARTH_X, 2, 8, HEARTH_GAMMA)
}

#[cfg(test)]
mod tests {
    use super::*;
    use zen_cooker_core::quant_tern::Trit;

    #[test]
    fn canonical_output_matches_golden() {
        let y = hearth_canonical();
        for (got, want) in y.iter().zip(HEARTH_GOLDEN.iter()) {
            assert!((got - want).abs() < 1e-5, "got {got}, want {want}");
        }
    }

    #[test]
    fn hand_verified_rows() {
        // Row 0: 1·3 + 0·7 − 1·2 + 1·9 + 0·1 + 1·4 − 1·6 + 1·5 = 13 -> 6.5
        // Row 1: −1·3 + 1·7 + 1·2 + 0·9 − 1·1 + 1·4 + 0·6 − 1·5 = 4 -> 2.0
        let y = hearth_canonical();
        let acc0: i32 = HEARTH_W[..8]
            .iter()
            .zip(HEARTH_X.iter())
            .map(|(&w, &x)| w as i32 * x as i32)
            .sum();
        let acc1: i32 = HEARTH_W[8..]
            .iter()
            .zip(HEARTH_X.iter())
            .map(|(&w, &x)| w as i32 * x as i32)
            .sum();
        assert!((y[0] - acc0 as f32 * HEARTH_GAMMA).abs() < 1e-5);
        assert!((y[1] - acc1 as f32 * HEARTH_GAMMA).abs() < 1e-5);
        assert!((y[0] - HEARTH_GOLDEN[0]).abs() < 1e-5);
        assert!((y[1] - HEARTH_GOLDEN[1]).abs() < 1e-5);
    }

    #[test]
    fn trit_form_agrees_with_core_matvec() {
        let trits: Vec<Trit> = HEARTH_W
            .iter()
            .map(|&w| Trit::from_f32(w as f32, 0.5))
            .collect();
        let via_core =
            zen_cooker_core::quant_tern::ternary_matvec(&trits, &HEARTH_X, 2, 8, HEARTH_GAMMA);
        let via_ref = hearth_canonical();
        assert!((via_core[0] - via_ref[0]).abs() < 1e-5);
        assert!((via_core[1] - via_ref[1]).abs() < 1e-5);
    }
}
