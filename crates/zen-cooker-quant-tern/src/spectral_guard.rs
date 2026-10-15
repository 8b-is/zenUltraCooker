//! Marchenko-Pastur spectral rigidity guard (KOMPRESS v2).
//!
//! For a ternary projection matrix `W ∈ {-1, 0, +1}^{m×n}` with non-zero
//! density `p` and aspect ratio `γ = m/n`, the empirical spectral
//! distribution of `(1/n) W W^T` converges to Marchenko-Pastur (shape `γ`,
//! scale `p`), and
//!
//! ```text
//! s_max(W) ≤ sqrt(p·n) · (1 + sqrt(γ)) + O(n^{-1/6})
//! ```
//!
//! The guard estimates `s_max(W)` by power iteration on `W^T W` — computed
//! with the ternary trick, zero multiplies in the inner loop — and refuses
//! the `BIND` stage when the estimate clears the bound plus the finite-`n`
//! slack term. A matrix that violates the bound is the fingerprint of a
//! collapsed representation (high-entropy tokens folding onto one mode),
//! which is exactly what must not reach the recipe or affect lattice.

use zen_cooker_core::quant_tern::Trit;

/// The Marchenko-Pastur bound: `sqrt(p·n) · (1 + sqrt(γ))`.
pub fn mp_bound(density: f32, n_cols: usize, aspect: f32) -> f32 {
    let p = density.clamp(0.0, 1.0);
    (p * n_cols as f32).sqrt() * (1.0 + aspect.max(0.0).sqrt())
}

/// Fixed-point scale for the gram step: `v` is carried as `i32 = v · 2^16`
/// so the ternary trick (conditional add/subtract, no multiplies) survives
/// the integer inner loop; the round trip divides once per element.
const FX: f64 = 1.0 / ((1i64 << 16) as f64);

/// One `W^T W v` step for `v ∈ R^n` — fixed-point `i64` accumulation,
/// conditional adds only in the trit loop (the ternary trick).
fn gram_step(w: &[Trit], m: usize, n: usize, v: &[f32]) -> Vec<f32> {
    let vf: Vec<i32> = v.iter().map(|&x| (x * (1 << 16) as f32) as i32).collect();
    let mut out = vec![0i64; n];
    for row in 0..m {
        let row_w = &w[row * n..(row + 1) * n];
        let mut dot = 0i64;
        // Ternary trick: accumulate ±v_fixed, never multiply.
        for (&t, &vc) in row_w.iter().zip(vf.iter()) {
            dot += if t == Trit::Plus {
                vc as i64
            } else if t == Trit::Minus {
                -(vc as i64)
            } else {
                0
            };
        }
        // Second matvec stage: out carries one more fixed-point factor
        // (a shift, still no multiply in the trit loop).
        for (&t, o) in row_w.iter().zip(out.iter_mut()) {
            *o += if t == Trit::Plus {
                dot << 16
            } else if t == Trit::Minus {
                -(dot << 16)
            } else {
                0
            };
        }
    }
    out.iter().map(|&x| (x as f64 * FX * FX) as f32).collect()
}

/// Power-iteration estimate of `s_max(W)` = `sqrt(λ_max(W^T W))`.
pub fn spectral_smax(w: &[Trit], m: usize, n: usize, iterations: usize) -> f32 {
    if m == 0 || n == 0 {
        return 0.0;
    }
    let mut v: Vec<f32> = (0..n).map(|i| ((i as f32) * 0.618 + 0.5).sin()).collect();
    let mut lambda = 0.0f32;
    for _ in 0..iterations {
        let next = gram_step(w, m, n, &v);
        let norm: f32 = next.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm <= 1e-12 {
            return 0.0;
        }
        // Rayleigh quotient against the PRE-update unit vector: v · next.
        lambda = next.iter().zip(&v).map(|(a, b)| a * b).sum::<f32>();
        for (x, nx) in v.iter_mut().zip(&next) {
            *x = *nx / norm;
        }
    }
    lambda.max(0.0).sqrt()
}

/// Non-zero density of a ternary matrix: fraction of `±1` entries.
pub fn density(w: &[Trit]) -> f32 {
    if w.is_empty() {
        return 0.0;
    }
    let nz = w.iter().filter(|t| **t != Trit::Zero).count();
    nz as f32 / w.len() as f32
}

/// The guard's verdict for a `BIND` stage.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GuardVerdict {
    /// Power-iteration estimate of the largest singular value.
    pub smax: f32,
    /// The Marchenko-Pastur bound at this shape and density.
    pub bound: f32,
    /// The finite-`n` relaxation actually applied (`slack_const · n^{-1/6}`).
    pub slack: f32,
    /// `true` when `smax ≤ bound + slack` — the representation is rigid,
    /// the `BIND` may proceed.
    pub allowed: bool,
}

/// Check a ternary matrix against the rigidity bound before `BIND`.
///
/// `slack_const` is the finite-`n` relaxation constant for the
/// `O(n^{-1/6})` term. The paper states the asymptotic; small matrices
/// need a concrete constant, and this crate names it (default 0.35) instead
/// of pretending the asymptote is exact at n = 16.
pub fn guard_bind(w: &[Trit], m: usize, n: usize, slack_const: f32) -> GuardVerdict {
    let p = density(w);
    let gamma = m as f32 / n.max(1) as f32;
    let bound = mp_bound(p, n, gamma);
    let slack = slack_const * (n.max(1) as f32).powf(-1.0 / 6.0);
    let smax = spectral_smax(w, m, n, 64);
    GuardVerdict {
        smax,
        bound,
        slack,
        allowed: smax <= bound + slack,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deterministic ternary matrix from a seed.
    fn seeded_matrix(seed: u64, m: usize, n: usize) -> Vec<Trit> {
        let mut state = seed;
        (0..m * n)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                match (state >> 62) % 3 {
                    0 => Trit::Minus,
                    1 => Trit::Zero,
                    _ => Trit::Plus,
                }
            })
            .collect()
    }

    #[test]
    fn random_ternary_matrix_is_rigid() {
        let (m, n) = (16, 16);
        let w = seeded_matrix(42, m, n);
        let verdict = guard_bind(&w, m, n, 0.35);
        assert!(
            verdict.allowed,
            "smax {} vs bound {} + slack {}",
            verdict.smax, verdict.bound, verdict.slack
        );
    }

    #[test]
    fn all_plus_matrix_violates_and_is_refused() {
        // W = ones(9×9): W^T W = 9·J, λ_max = 81, s_max = 9.
        // Bound at p=1, γ=1: sqrt(9)·(1+1) = 6. Violation must be caught.
        let (m, n) = (9, 9);
        let w = vec![Trit::Plus; m * n];
        let verdict = guard_bind(&w, m, n, 0.35);
        assert!(
            !verdict.allowed,
            "collapsed matrix must be refused: {verdict:?}"
        );
        assert!(
            (verdict.smax - 9.0).abs() < 0.5,
            "smax was {}",
            verdict.smax
        );
    }

    #[test]
    fn bound_matches_closed_form() {
        // p = 1/3, n = 36, γ = 2: sqrt(12)·(1 + sqrt(2))
        let expected = (36.0f32 / 3.0).sqrt() * (1.0 + 2.0f32.sqrt());
        assert!((mp_bound(1.0 / 3.0, 36, 2.0) - expected).abs() < 1e-6);
    }

    #[test]
    fn density_counts_nonzero_only() {
        let w = vec![Trit::Plus, Trit::Zero, Trit::Minus, Trit::Zero];
        assert!((density(&w) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn empty_matrix_is_zero_and_allowed() {
        let verdict = guard_bind(&[], 0, 0, 0.35);
        assert_eq!(verdict.smax, 0.0);
        assert!(verdict.allowed);
    }
}
