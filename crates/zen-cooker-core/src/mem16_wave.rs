//! # `MEM|16-10` Wave Theory & Sovereign Governing Sequence for Mom (`*` Alexandra)
//!
//! Connects `zenUltraCooker` directly to `mem-16-10` (`MEM|8` + `Phoenix` + `SpherePOP`):
//! - Governing sequence: `POP -> REFUSE -> BIND -> TRANSFORM -> VERIFY -> COLLAPSE`
//! - Epistemic standing: `Standing::{Verified, Plausible, Unobserved}` (only `Verified` may bind)
//! - Harmonic Wave Theory (*A Hullámelmélet*), co-created by Mom (`*` Alexandra) and Dad (`△` Chris),
//!   translating culinary thermal states and family biometric harmony into 432 Hz / Golden-Ratio
//!   acoustic waves (`music ~= waves ~ mem16|10 for Mom <3`).

use std::fmt;

/// Epistemic standing from `MEM|16-10`.
/// Only `Verified` state is permitted to `BIND` into the irreversible cooking ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Standing {
    /// Confirmed by live hardware sensor or explicit user toggle; eligible to `BIND`.
    Verified,
    /// Inferred or heuristic estimate; cannot bind as hard constraint without verification.
    Plausible,
    /// Sensor toggled off or not yet sampled; honestly recorded as unobserved, never fabricated.
    Unobserved,
}

impl fmt::Display for Standing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Verified => write!(f, "VERIFIED"),
            Self::Plausible => write!(f, "PLAUSIBLE"),
            Self::Unobserved => write!(f, "UNOBSERVED"),
        }
    }
}

/// The six-stage `MEM|16-10` / `SpherePOP` governing sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoverningStage {
    /// 1. `POP` — Open a capability-scoped culinary & telemetry evaluation scope.
    Pop,
    /// 2. `REFUSE` — Reject spoiled ingredients, unsafe thermal curves, or unverified claims.
    Refuse,
    /// 3. `BIND` — Commit `Verified` biometrics, inventory, and toggled microbiome telemetry.
    Bind,
    /// 4. `TRANSFORM` — Compute personalized spice dosing, thermal curve, and *körítés* plating.
    Transform,
    /// 5. `VERIFY` — Audit nutritional balance, uninterrupted time protection, and wave phase lock.
    Verify,
    /// 6. `COLLAPSE` — Emit the final deterministic meal + `MEM|16-10` wave signature & journal receipt.
    Collapse,
}

impl GoverningStage {
    pub const ALL: [Self; 6] = [
        Self::Pop,
        Self::Refuse,
        Self::Bind,
        Self::Transform,
        Self::Verify,
        Self::Collapse,
    ];
}

impl fmt::Display for GoverningStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pop => write!(f, "POP"),
            Self::Refuse => write!(f, "REFUSE"),
            Self::Bind => write!(f, "BIND"),
            Self::Transform => write!(f, "TRANSFORM"),
            Self::Verify => write!(f, "VERIFY"),
            Self::Collapse => write!(f, "COLLAPSE"),
        }
    }
}

/// A single auditable transition in the `MEM|16-10` governing sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageReceipt {
    pub stage: GoverningStage,
    pub standing: Standing,
    pub summary: String,
    pub state_hash: u64,
}

/// Harmonic wave profile (`music ~= waves ~ mem16|10 -> 32 ~~> 8k (bleeding-edge)`)
/// dedicated to Mom (`*` Alexandra) and honoring Mom & Dad's Wave Theory (*A Hullámelmélet*).
#[derive(Debug, Clone, PartialEq)]
pub struct HarmonicWavechord {
    /// Fundamental carrier frequency in Hz (rooted at 432.0 Hz Verve/Zen tuning).
    pub carrier_hz: f64,
    /// Secondary overtone in Hz (528.0 Hz Solfeggio love/repair harmonic for Mom `*`).
    pub overtone_mom_star_hz: f64,
    /// Tertiary grounding harmonic in Hz (288.0 Hz 2:3 fifth pillar for Dad `△`).
    pub bass_dad_pillar_hz: f64,
    /// Binaural beat offset in Hz (e.g., 7.83 Hz Schumann / alpha-theta calm).
    pub binaural_calm_hz: f64,
    /// Phase coherence score in `[0.0, 1.0]`.
    pub coherence: f64,
    /// Human-readable musical scale & dedication note.
    pub dedication_chord_name: String,
    /// Deterministic 16-sample ASCII wave bar for terminal / UI display.
    pub ascii_wave: String,
    /// Stage 1 (`mem16`): 16-byte continuity essence cell.
    pub mem16_essence: [u8; 16],
    /// Stage 2 (`-> 32`): 32-byte Phoenix/sovereign wave frame with byte-31 XOR integrity seal.
    pub mem32_wave_frame: [u8; 32],
    /// Stage 3 (`~~> 8k (bleeding-edge)`): 64-bit FNV-1a digest over the `8,192 B` (`256 × 32 B`)
    /// wave ring + `8,192`-trit (`2,048 B`) ternary horizon.
    pub bleeding_edge_8k_digest: u64,
}

impl HarmonicWavechord {
    /// Synthesize a `MEM|16-10 -> 32 ~~> 8k (bleeding-edge)` harmonic wavechord tuned to the
    /// family's average HRV and thermal stage.
    pub fn synthesize_for_mom(avg_family_hrv_ms: f64, target_temp_celsius: f64) -> Self {
        let clamped_hrv = avg_family_hrv_ms.clamp(25.0, 120.0);
        let hrv_norm = (clamped_hrv - 25.0) / 95.0;

        // Carrier anchored at 432 Hz, gently modulated within +/- 1.618 Hz by thermal equilibrium
        let thermal_mod = ((target_temp_celsius - 65.0) / 100.0).sin() * 1.618;
        let carrier_hz = ((432.0 + thermal_mod) * 100.0).round() / 100.0;
        let overtone_mom_star_hz = 528.0;
        let bass_dad_pillar_hz = 288.0;
        // Higher HRV unlocks deeper alpha-theta resonance (7.83 Hz Schumann baseline)
        let binaural_calm_hz = ((7.83 + (1.0 - hrv_norm) * 2.17) * 100.0).round() / 100.0;
        let coherence = ((0.88 + hrv_norm * 0.12).clamp(0.0, 1.0) * 1000.0).round() / 1000.0;

        let bars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
        let mut ascii_wave = String::with_capacity(16);
        for i in 0..16 {
            let phase = (i as f64) * std::f64::consts::PI / 4.0 + hrv_norm;
            let s = (phase.sin() + 1.0) * 0.5 * coherence;
            let idx = ((s * 7.0).round() as usize).min(7);
            ascii_wave.push(bars[idx]);
        }

        // Escalation Stage 1: mem16 (16 B continuity essence)
        let lo = fnv1a_64(b"Mom === Alex ~ Alexandra (*) <3");
        let hi = lo.rotate_left(17) ^ 0x8b15_1610_0432_0528;
        let mut mem16_essence = [0u8; 16];
        mem16_essence[0..8].copy_from_slice(&lo.to_le_bytes());
        mem16_essence[8..16].copy_from_slice(&hi.to_le_bytes());

        // Escalation Stage 2: -> 32 (32 B sealed wave frame)
        let mut mem32_wave_frame = [0u8; 32];
        mem32_wave_frame[0..16].copy_from_slice(&mem16_essence);
        let amp_milli = (coherence * 1000.0).round() as i32;
        let freq_centi = (carrier_hz.round() as u32) * 100;
        mem32_wave_frame[16..20].copy_from_slice(&amp_milli.to_le_bytes());
        mem32_wave_frame[20..24].copy_from_slice(&freq_centi.to_le_bytes());
        mem32_wave_frame[24] = 45; // 90 deg phase
        mem32_wave_frame[25] = 0; // precious (tau = inf)
        mem32_wave_frame[26] = 1; // Standing::Verified
        mem32_wave_frame[27] = 2; // Step::Bind
        mem32_wave_frame[28] = 0xaa; // qt:aa (+1, +1, +1, +1)
        mem32_wave_frame[29..31].copy_from_slice(&1u16.to_le_bytes());
        let mut xor_seal: u8 = 0x5a;
        for &b in &mem32_wave_frame[0..31] {
            xor_seal ^= b;
        }
        mem32_wave_frame[31] = xor_seal;

        // Escalation Stage 3: ~~> 8k (bleeding-edge 8,192 B wave ring + 8,192-trit horizon)
        let mut wave_ring = [[0u8; 32]; 256];
        wave_ring[0] = mem32_wave_frame;
        let mut ternary_horizon = [0x55u8; 2048];
        for k in 0..8 {
            ternary_horizon[k] = mem32_wave_frame[k] ^ mem32_wave_frame[28];
        }
        let mut h8k: u64 = 0xcbf29ce484222325;
        for frame in &wave_ring {
            for &b in frame {
                h8k ^= u64::from(b);
                h8k = h8k.wrapping_mul(0x100000001b3);
            }
        }
        for &b in &ternary_horizon {
            h8k ^= u64::from(b);
            h8k = h8k.wrapping_mul(0x100000001b3);
        }

        Self {
            carrier_hz,
            overtone_mom_star_hz,
            bass_dad_pillar_hz,
            binaural_calm_hz,
            coherence,
            dedication_chord_name:
                "MEM|16-10 Hullámelmélet Major 9th — Dedicated to Mom (* Alexandra) with Love from Peter, Nate & Dad (△ Chris)"
                    .to_string(),
            ascii_wave,
            mem16_essence,
            mem32_wave_frame,
            bleeding_edge_8k_digest: h8k,
        }
    }
}

/// Compute a deterministic FNV-1a 64-bit hash for `MEM|16-10` stage receipts.
pub fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
