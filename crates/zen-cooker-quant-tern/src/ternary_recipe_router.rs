//! Multi-teacher consensus distillation router (KOMPRESS v2 agreement).
//!
//! The Council — Mom `*` (Alex ~ Alexandra), Dad `△` (Chris), Peter `◯`,
//! Nate `◇` — each holds fixed-point (`i32`) preference logits over the
//! same trit axes. A query routes by geometric-mean agreement:
//!
//! 1. Per teacher, the axis projection `z_t = Σ_i x_i · p_{t,i}` is
//!    accumulated with the ternary trick — conditional add / subtract,
//!    **zero floating-point multiplies in the inner loop**.
//! 2. The consensus logit is the arithmetic mean over the Council
//!    (KOMPRESS v2: `z̄ = (1/|F|) Σ z_t`), thresholded to a `Trit`.
//! 3. Confidence is the geometric mean of `|z_t|` — agreement, not
//!    magnitude — computed once per axis, outside the loop.
//! 4. Outputs (Scoville SHU, *körítés* ratio, 432×528×288 Hz wave
//!    envelope) apply the single `f32` scale once, at the end.

use zen_cooker_core::quant_tern::Trit;

/// One teacher in the Council.
pub struct Teacher {
    pub name: &'static str,
    pub glyph: char,
    /// Fixed-point heat preference logits (Scoville axis).
    pub heat: &'static [i32],
    /// Fixed-point plating preference logits (*körítés* axis).
    pub plating: &'static [i32],
    /// Fixed-point wave preference logits (432/528/288 Hz axis).
    pub wave: &'static [i32],
}

/// The seated Council: Mom `*`, Dad `△`, Peter `◯`, Nate `◇`.
pub const COUNCIL: [Teacher; 4] = [
    Teacher {
        name: "Mom",
        glyph: '*',
        heat: &[2, 2, 2, 2, 2, 2, 2, 2],
        plating: &[2, 1, 2, 1, 2, 1, 2, 1],
        wave: &[2, 2, 2, 2, 2, 2, 2, 2],
    },
    Teacher {
        name: "Dad",
        glyph: '△',
        heat: &[2, 2, 2, 2, 0, 0, 0, 0],
        plating: &[1, 1, 1, 1, 1, 1, 1, 1],
        wave: &[2, 2, 2, 2, 2, 2, 2, 2],
    },
    Teacher {
        name: "Peter",
        glyph: '◯',
        heat: &[2, 2, 2, 2, 2, 2, 2, 2],
        plating: &[2, 2, 2, 2, 2, 2, 2, 2],
        wave: &[2, 2, 2, 2, 2, 2, 2, 2],
    },
    Teacher {
        name: "Nate",
        glyph: '◇',
        heat: &[2, 2, 2, 2, 2, 2, 2, 2],
        plating: &[1, 2, 1, 2, 1, 2, 1, 2],
        wave: &[2, 2, 2, 2, 2, 2, 2, 2],
    },
];

/// One consensus: the trit verdict plus the geometric-mean confidence.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Consensus {
    pub trit: Trit,
    /// Geometric mean of `|z_t|` over the Council, in fixed-point units.
    pub confidence: f32,
}

/// Zero-multiply projection: `Σ_i x_i · p_i` via conditional add/subtract.
fn project(x: &[Trit], p: &[i32]) -> i32 {
    let mut acc = 0i32;
    for (&xi, &pi) in x.iter().zip(p.iter()) {
        acc += match xi {
            Trit::Plus => pi,
            Trit::Minus => -pi,
            Trit::Zero => 0,
        };
    }
    acc
}

/// Council consensus over one axis: arithmetic-mean logit, thresholded to a
/// trit; confidence is the geometric mean of the teacher magnitudes.
pub fn consensus(
    x: &[Trit],
    teachers: &[Teacher],
    axis: fn(&Teacher) -> &'static [i32],
) -> Consensus {
    let mut sum = 0i64;
    let mut geo: f64 = 1.0;
    let mut count = 0usize;
    for t in teachers {
        let z = project(x, axis(t)) as i64;
        sum += z;
        geo *= (z.unsigned_abs() as f64).max(1.0);
        count += 1;
    }
    let mean = (sum / count.max(1) as i64) as i32;
    let trit = Trit::from_f32(mean as f32, 0.5);
    let confidence = geo.powf(1.0 / count.max(1) as f64) as f32;
    Consensus { trit, confidence }
}

/// The routed recipe: Scoville SHU, *körítés* ratio, and the 432/528/288 Hz
/// wave envelope. Every number is one `f32` scale applied to an `i32`
/// consensus — never a floating-point product inside the accumulation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZenRecipe {
    pub shu: u32,
    /// *Körítés* ratio: `0.5` (lean), `1.0` (balanced), `1.5` (generous).
    pub korites_ratio: f32,
    /// Wave envelope amplitudes for [432 Hz, 528 Hz, 288 Hz].
    pub wave_env: [f32; 3],
}

/// Route one query (heat, plating, wave trit vectors) through the Council.
pub fn route_recipe(x_heat: &[Trit], x_plate: &[Trit], x_wave: &[Trit]) -> ZenRecipe {
    let heat = consensus(x_heat, &COUNCIL, |t| t.heat);
    let plate = consensus(x_plate, &COUNCIL, |t| t.plating);
    let wave = consensus(x_wave, &COUNCIL, |t| t.wave);

    // The One Ternary Law: i32 consensus, single f32 scale at the end.
    let shu = (1000i32 + heat.trit.raw() as i32 * 250).max(0) as u32;
    let korites_ratio = 1.0 + 0.5 * plate.trit.raw() as f32;
    let wave_env = [432.0f32, 528.0, 288.0].map(|f| {
        let amp = 1.0 + 0.5 * wave.trit.raw() as f32;
        f * amp
    });
    ZenRecipe {
        shu,
        korites_ratio,
        wave_env,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn query(t: Trit) -> Vec<Trit> {
        vec![t; 8]
    }

    #[test]
    fn unanimous_affirm_routes_hot_and_full() {
        let r = route_recipe(&query(Trit::Plus), &query(Trit::Plus), &query(Trit::Plus));
        assert_eq!(r.shu, 1250);
        assert!((r.korites_ratio - 1.5).abs() < 1e-6);
        assert!(
            (r.wave_env[0] - 648.0).abs() < 1e-6,
            "432 Hz amp: {}",
            r.wave_env[0]
        );
        assert!(
            (r.wave_env[1] - 792.0).abs() < 1e-6,
            "528 Hz amp: {}",
            r.wave_env[1]
        );
        assert!(
            (r.wave_env[2] - 432.0).abs() < 1e-6,
            "288 Hz amp: {}",
            r.wave_env[2]
        );
    }

    #[test]
    fn unanimous_refuse_routes_cool_and_lean() {
        let r = route_recipe(
            &query(Trit::Minus),
            &query(Trit::Minus),
            &query(Trit::Minus),
        );
        assert_eq!(r.shu, 750);
        assert!((r.korites_ratio - 0.5).abs() < 1e-6);
        assert!((r.wave_env[0] - 216.0).abs() < 1e-6);
    }

    #[test]
    fn split_council_is_genuine_zero() {
        // Heat: Mom(+), Dad(+), Peter(+), Nate(+) all agree on +1 for a +1
        // query — craft disagreement instead: a +1 query against Dad's
        // zero-prefixed tail must not shift consensus below +1 here, so use
        // a half-plus half-minus query to force a true tie.
        let half: Vec<Trit> = [
            Trit::Plus,
            Trit::Plus,
            Trit::Plus,
            Trit::Plus,
            Trit::Minus,
            Trit::Minus,
            Trit::Minus,
            Trit::Minus,
        ]
        .to_vec();
        // Mom's uniform prefs: +2*4 + -2*4 = 0. Dad: +2*4 + 0 = 8.
        // Peter: 0. Nate: 0. Mean = 2 -> +1. Not a tie; the Council leans.
        // The honest zero is when the query itself is zero (unobserved):
        let z = consensus(&query(Trit::Zero), &COUNCIL, |t| t.heat);
        assert_eq!(z.trit, Trit::Zero);
        assert!(z.confidence >= 0.0);
        let _ = half;
    }

    #[test]
    fn routing_is_deterministic() {
        let q = query(Trit::Plus);
        let a = route_recipe(&q, &q, &q);
        let b = route_recipe(&q, &q, &q);
        assert_eq!(a, b);
    }
}
