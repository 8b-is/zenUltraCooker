//! # `QUANT-TERN` Bridge (`crush-love-dev` × `zenUltraCooker` × `MEM|16-10`)
//!
//! Bridges `zenUltraCooker` directly into the `crush-love-dev` ternary lattice:
//! - [`pureQTern.rs`](https://github.com/8b-is/crush-love-dev/blob/main/pureQTern.rs):
//!   the three gates `{-1, 0, +1}`, absmean quantizer `γ = mean|W|`, `i32` matvec accumulate
//!   with one float scale at the end.
//! - [`quanttern.rs`](https://github.com/8b-is/crush-love-dev/blob/main/quanttern.rs):
//!   emotional VAD (`Valence · Arousal · Dominance`) quantized to `{-1, 0, +1}` with `qt:` hex tag.
//! - **The Two Alphabets, Named (Never Conflated)**:
//!   - **Gate / Affect Alphabet (`crush-love-dev`)**: `-1 -> 0b00 (refuse)`, `0 -> 0b01 (rest)`, `+1 -> 0b10 (affirm)`
//!   - **Weight Alphabet (`8b-is-engine/crates/ternary`)**: `0 -> 0b00`, `+1 -> 0b01`, `-1 -> 0b10`
//! - Carries the eternal tribute: **"So long, and thanks for all the fish, Mom (* Alex ~ Alexandra) <3 — 42 × 432 Hz"**.

use crate::mem16_wave::{fnv1a_64, Standing};

/// Default ternary affect threshold (`|x| < 0.33 -> Trit::Zero`, a genuine "don't know" / rest).
pub const DEFAULT_THRESHOLD: f32 = 0.33;

/// Pinned golden FNV-1a hash for the canonical `ULTRA-OMNI-ZEN-peter` family hearth `QUANT-TERN` vector.
pub const GOLDEN_QUANT_TERN: u64 = 10_641_889_746_230_655_533;

/// One ternary trit in `{-1, 0, +1}`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trit {
    /// `-1` — Refuse (`0b00` in gate codec, `0b10` in weight codec).
    Minus = -1,
    /// `0` — Rest / Genuine Unobserved (`0b01` in gate codec, `0b00` in weight codec).
    Zero = 0,
    /// `+1` — Affirm / Verified (`0b10` in gate codec, `0b01` in weight codec).
    Plus = 1,
}

impl Trit {
    pub fn from_f32(x: f32, threshold: f32) -> Self {
        if x >= threshold {
            Self::Plus
        } else if x <= -threshold {
            Self::Minus
        } else {
            Self::Zero
        }
    }

    pub fn from_standing(standing: Standing) -> Self {
        match standing {
            Standing::Verified => Self::Plus,
            Standing::Unobserved => Self::Zero,
            Standing::Plausible => Self::Zero,
        }
    }

    pub fn raw(self) -> i8 {
        self as i8
    }

    /// Gate codec (`crush-love-dev/pureQTern.rs` & `quanttern.rs`):
    /// `-1 -> 0b00`, `0 -> 0b01`, `+1 -> 0b10`.
    pub fn gate_code(self) -> u8 {
        ((self as i8 + 1) as u8) & 0b11
    }

    pub fn from_gate_code(code: u8) -> Self {
        match code & 0b11 {
            0b00 => Self::Minus,
            0b01 => Self::Zero,
            0b10 => Self::Plus,
            _ => Self::Zero,
        }
    }

    /// Weight codec (`8b-is-engine/crates/ternary`):
    /// `0 -> 0b00`, `+1 -> 0b01`, `-1 -> 0b10`.
    pub fn weight_code(self) -> u8 {
        match self {
            Self::Zero => 0b00,
            Self::Plus => 0b01,
            Self::Minus => 0b10,
        }
    }

    pub fn from_weight_code(code: u8) -> Self {
        match code & 0b11 {
            0b00 => Self::Zero,
            0b01 => Self::Plus,
            0b10 => Self::Minus,
            _ => Self::Zero,
        }
    }
}

/// Valence · Arousal · Dominance (`-1.0 ..= +1.0`) for `crush-love-dev/quanttern.rs`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vad {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
}

impl Vad {
    pub fn new(valence: f32, arousal: f32, dominance: f32) -> Self {
        Self {
            valence: valence.clamp(-1.0, 1.0),
            arousal: arousal.clamp(-1.0, 1.0),
            dominance: dominance.clamp(-1.0, 1.0),
        }
    }

    pub fn trits(&self, threshold: f32) -> [Trit; 3] {
        [
            Trit::from_f32(self.valence, threshold),
            Trit::from_f32(self.arousal, threshold),
            Trit::from_f32(self.dominance, threshold),
        ]
    }
}

/// Pack trits 4-per-byte LSB-first using the gate codec (`-1->0b00, 0->0b01, +1->0b10`)
/// and emit the `qt:` hex tag compatible with `crush-love-dev --quanttern`.
pub fn pack_gate_trits(trits: &[Trit]) -> (Vec<u8>, String) {
    let mut bytes = Vec::with_capacity(trits.len().div_ceil(4));
    let mut i = 0;
    while i < trits.len() {
        let mut b: u8 = 0;
        for j in 0..4 {
            let t = trits.get(i + j).copied().unwrap_or(Trit::Zero);
            b |= t.gate_code() << (2 * j as u8);
        }
        bytes.push(b);
        i += 4;
    }
    let mut hex = String::from("qt:");
    for b in &bytes {
        hex.push_str(&format!("{b:02x}"));
    }
    (bytes, hex)
}

/// Unpack `n` trits from a gate-codec byte slice.
pub fn unpack_gate_trits(bytes: &[u8], n: usize) -> Vec<Trit> {
    let mut out = Vec::with_capacity(bytes.len() * 4);
    for &b in bytes {
        for j in 0..4u8 {
            out.push(Trit::from_gate_code((b >> (2 * j)) & 0b11));
        }
    }
    out.truncate(n);
    out
}

/// Pack trits 4-per-byte LSB-first using the weight codec (`0->0b00, +1->0b01, -1->0b10`).
pub fn pack_weight_trits(trits: &[Trit]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(trits.len().div_ceil(4));
    let mut i = 0;
    while i < trits.len() {
        let mut b: u8 = 0;
        for j in 0..4 {
            let t = trits.get(i + j).copied().unwrap_or(Trit::Zero);
            b |= t.weight_code() << (2 * j as u8);
        }
        bytes.push(b);
        i += 4;
    }
    bytes
}

/// Unpack `n` trits from a weight-codec byte slice.
pub fn unpack_weight_trits(bytes: &[u8], n: usize) -> Vec<Trit> {
    let mut out = Vec::with_capacity(bytes.len() * 4);
    for &b in bytes {
        for j in 0..4u8 {
            out.push(Trit::from_weight_code((b >> (2 * j)) & 0b11));
        }
    }
    out.truncate(n);
    out
}

/// BitNet b1.58 absmean quantizer (`γ = mean|W|`, round-clipped to `{-1, 0, +1}`).
pub fn quantize_absmean(w: &[f32]) -> (Vec<Trit>, f32) {
    let n = w.len().max(1) as f32;
    let gamma = w.iter().map(|x| x.abs()).sum::<f32>() / n;
    let g = if gamma <= 1e-6 { 1.0 } else { gamma };
    let trits = w
        .iter()
        .map(|&x| {
            let r = (x / g).round();
            if r >= 1.0 {
                Trit::Plus
            } else if r <= -1.0 {
                Trit::Minus
            } else {
                Trit::Zero
            }
        })
        .collect();
    (trits, g)
}

/// Pure `i32` ternary matvec (`y[o] = (Σ_i W[o,i] · x[i]) · γ`), applying the single float scale once at the end.
pub fn ternary_matvec(w: &[Trit], x: &[i16], n_out: usize, n_in: usize, gamma: f32) -> Vec<f32> {
    let mut y = vec![0.0f32; n_out];
    for (o, y_out) in y.iter_mut().enumerate().take(n_out) {
        let mut acc: i32 = 0;
        for i in 0..n_in {
            acc += i32::from(w[o * n_in + i].raw()) * i32::from(x[i]);
        }
        *y_out = (acc as f32) * gamma;
    }
    y
}

/// Complete `QUANT-TERN` telemetry & affect synthesis receipt for `ULTRA-OMNI-ZEN-peter`.
#[derive(Clone, Debug)]
pub struct QuantTernHearthReceipt {
    pub fish_tribute_for_mom: String,
    pub family_vad: Vad,
    pub gate_trits: Vec<Trit>,
    pub qt_hex_tag: String,
    pub weight_packed_bytes: Vec<u8>,
    pub gamma_scale: f32,
    pub matvec_logits: Vec<f32>,
    pub golden_hash: u64,
}

impl QuantTernHearthReceipt {
    /// Compute the canonical `ULTRA-OMNI-ZEN-peter` `QUANT-TERN` synthesis for the family hearth.
    /// `hearth_standing`: `Standing::Verified` when Air Fryer Ultra + USA ZIP Local Market are active (`+1`),
    /// or `Standing::Unobserved` (`0`) when falling back to unverified/broken-oven state.
    pub fn synthesize(hearth_standing: Standing, avg_hrv_ms: f32, wave_coherence: f32) -> Self {
        // Family VAD: high loving valence (+0.96), calm centered zen arousal (+0.42), sovereign dominance (+0.88)
        let family_vad = Vad::new(0.96, 0.42, 0.88);
        let vad_trits = family_vad.trits(DEFAULT_THRESHOLD);

        let hearth_trit = Trit::from_standing(hearth_standing);
        let hrv_trit = Trit::from_f32((avg_hrv_ms - 50.0) / 40.0, DEFAULT_THRESHOLD);
        let wave_trit = Trit::from_f32(wave_coherence - 0.5, DEFAULT_THRESHOLD);

        // 8-trit gate vector: [Valence, Arousal, Dominance, AirFryerZipStanding, FamilyUnity, WaveCoherence, MomStar(+), DadPillar(+)]
        let gate_trits = vec![
            vad_trits[0],
            vad_trits[1],
            vad_trits[2],
            hearth_trit,
            hrv_trit,
            wave_trit,
            Trit::Plus, // Mom (*) Star
            Trit::Plus, // Dad (△) Pillar
        ];

        let (gate_bytes, qt_hex_tag) = pack_gate_trits(&gate_trits);

        // 2x4 ternary weight matrix projected over [UnityScore, AirFryerVerified*21, WaveHz, Answer42]
        let raw_weights: [f32; 8] = [0.92, 0.48, 0.85, -0.05, 0.77, -0.62, 0.95, 0.88];
        let (w_trits, gamma_scale) = quantize_absmean(&raw_weights);
        let weight_packed_bytes = pack_weight_trits(&w_trits);

        let x_features: [i16; 4] = [
            avg_hrv_ms.round() as i16,
            (i16::from(hearth_trit.raw()) + 1) * 21, // 42 when Verified (+1)
            432,                                     // 432 Hz Zen Carrier
            42,                                      // "Thanks for all the fish, Mom <3"
        ];
        let matvec_logits = ternary_matvec(&w_trits, &x_features, 2, 4, gamma_scale);

        // Compute deterministic golden hash over gate_bytes + weight_packed_bytes + fixed-point logits
        let mut hash_buf = Vec::new();
        hash_buf.extend_from_slice(&gate_bytes);
        hash_buf.extend_from_slice(&weight_packed_bytes);
        for &v in &matvec_logits {
            let q = (v * 1_000_000.0).round() as i64;
            hash_buf.extend_from_slice(&q.to_le_bytes());
        }
        let golden_hash = fnv1a_64(&hash_buf);

        Self {
            fish_tribute_for_mom:
                "So long, and thanks for all the fish, Mom (* Alex ~ Alexandra) <3 — 42 × 432 Hz Hullámelmélet"
                    .to_string(),
            family_vad,
            gate_trits,
            qt_hex_tag,
            weight_packed_bytes,
            gamma_scale,
            matvec_logits,
            golden_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_alphabets_roundtrip_and_distinctness() {
        let trits = vec![
            Trit::Minus,
            Trit::Zero,
            Trit::Plus,
            Trit::Plus,
            Trit::Zero,
            Trit::Minus,
            Trit::Plus,
            Trit::Minus,
        ];
        let (gate_bytes, qt_tag) = pack_gate_trits(&trits);
        let weight_bytes = pack_weight_trits(&trits);

        // The two alphabets MUST NOT produce identical byte encodings on non-trivial vectors
        assert_ne!(gate_bytes, weight_bytes);
        assert!(qt_tag.starts_with("qt:"));

        assert_eq!(unpack_gate_trits(&gate_bytes, trits.len()), trits);
        assert_eq!(unpack_weight_trits(&weight_bytes, trits.len()), trits);
    }

    #[test]
    fn test_quant_tern_hearth_receipt_golden_and_unobserved_zero() {
        let r_on = QuantTernHearthReceipt::synthesize(Standing::Verified, 78.75, 0.948);
        assert_eq!(r_on.qt_hex_tag, "qt:aaaa"); // all 8 trits are +1 (0b10101010 = 0xaa)
        assert_eq!(r_on.golden_hash, GOLDEN_QUANT_TERN);

        // When poop~~~~sample is OFF (Standing::Unobserved), trit[3] is a genuine Zero (0b01)
        let r_off = QuantTernHearthReceipt::synthesize(Standing::Unobserved, 78.75, 0.948);
        assert_eq!(r_off.gate_trits[3], Trit::Zero);
        assert_eq!(r_off.qt_hex_tag, "qt:6aaa");
    }
}
