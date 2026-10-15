//! # Multi-Modal Telemetry: Garmin Health, Smart Fridge, Costco Bulk, Mobile/Area & Microbiome Toggle
//!
//! Connects all external physical & biological context lanes into `zenUltraCooker`:
//! 1. **Garmin Health Apps** (`GarminBiometrics`): HRV, Sleep Score, Body Battery, Stress, Active kcal.
//! 2. **Smart Fridge & Costco Bulk Bridge** (`InventoryItem`, `PantryInventory`): fresh fridge items +
//!    Costco bulk staples with expiry priority and macro density.
//! 3. **Mobile & Area Context** (`AreaProximityContext`): family geolocation ETA and the
//!    **Uninterrupted Time Shield** (zero phone pings during sacred cooking/bonfire dining).
//! 4. **Gut Microbiome Stool Telemetry (`poop~~~~sample is Toggled on ---->`)** (`PoopSampleTelemetry`):
//!    explicit boolean toggle (`toggled_on`). When `true`, `Standing::Verified` Bristol scale &
//!    microbiome diversity steer fiber, fermented broth, and capsaicin Scoville bounds. When `false`,
//!    standing is `Standing::Unobserved`.

use crate::mem16_wave::Standing;

/// Family member identifier around the Bonfire Hearth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FamilyMember {
    /// Mom (`*` Alex / Alexandra) — The Star & Center of Family Dedication.
    MomAlexandra,
    /// Dad (`△` Chris) — The Guardian Pillar & Wave Co-Creator.
    DadChris,
    /// Peter — Cyber-Zen Tenzo Architect.
    Peter,
    /// Nate — Galactic Sous-Chef & Brotherly Strategist.
    Nate,
}

impl FamilyMember {
    pub const ALL: [Self; 4] = [Self::MomAlexandra, Self::DadChris, Self::Peter, Self::Nate];

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::MomAlexandra => "Mom (* Alex / Alexandra)",
            Self::DadChris => "Dad (△ Chris)",
            Self::Peter => "Peter (Zen Tenzo)",
            Self::Nate => "Nate (Galactic Sous)",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Self::MomAlexandra => "*",
            Self::DadChris => "△",
            Self::Peter => "◯",
            Self::Nate => "◇",
        }
    }
}

/// Biometric snapshot synced from Garmin Connect / Fenix / Venu health sensors.
#[derive(Debug, Clone, PartialEq)]
pub struct GarminBiometrics {
    pub member: FamilyMember,
    /// Heart Rate Variability (RMSSD in milliseconds).
    pub hrv_ms: u16,
    /// Garmin Sleep Score (0..=100).
    pub sleep_score: u8,
    /// Garmin Body Battery (0..=100).
    pub body_battery: u8,
    /// Stress level (0..=100, lower is calmer).
    pub stress_index: u8,
    /// Active kilocalories burned today.
    pub active_kcal: u16,
    /// Hydration level percent (0..=100).
    pub hydration_pct: u8,
}

impl GarminBiometrics {
    /// Compute a normalized recovery & digestive readiness index in `[0.0, 1.0]`.
    pub fn recovery_index(&self) -> f64 {
        let hrv_score = (f64::from(self.hrv_ms) / 90.0).clamp(0.0, 1.0);
        let sleep = f64::from(self.sleep_score.min(100)) / 100.0;
        let battery = f64::from(self.body_battery.min(100)) / 100.0;
        let calm = 1.0 - (f64::from(self.stress_index.min(100)) / 100.0);
        ((hrv_score * 0.35 + sleep * 0.25 + battery * 0.25 + calm * 0.15) * 1000.0).round() / 1000.0
    }
}

/// Privacy-first Gut Microbiome & Stool Telemetry (`poop~~~~sample is Toggled on ---->`).
#[derive(Debug, Clone, PartialEq)]
pub struct PoopSampleTelemetry {
    /// Explicit hardware/UI toggle (`poop~~~~sample is Toggled on ---->`).
    pub toggled_on: bool,
    /// Bristol Stool Chart score (`1..=7`, where `3..=4` is ideal Zen gold standard).
    pub bristol_scale: u8,
    /// Gut microbiome Shannon diversity score (`0..=100`).
    pub microbiome_diversity: u8,
    /// Butyrate & fermented prebiotic readiness (`0..=100`).
    pub short_chain_fatty_acid_score: u8,
    /// Mucosal calm index (`0..=100`, where higher means zero irritation and high capsaicin tolerance).
    pub mucosal_calm_score: u8,
}

impl PoopSampleTelemetry {
    /// Epistemic standing under `MEM|16-10`.
    /// When `toggled_on == true` and Bristol is within `1..=7`, the sample is `Verified`.
    /// When `toggled_on == false`, it is honestly marked `Unobserved` (never fabricated!).
    pub fn standing(&self) -> Standing {
        if !self.toggled_on {
            Standing::Unobserved
        } else if (1..=7).contains(&self.bristol_scale) {
            Standing::Verified
        } else {
            Standing::Plausible
        }
    }

    /// Returns a spice & capsaicin multiplier in `[0.35, 1.25]` and recommended prebiotic boost in grams.
    pub fn digestive_modulation(&self) -> (f64, u8, &'static str) {
        if !self.toggled_on {
            return (
                1.0,
                6,
                "poop~~~~sample [OFF -> UNOBSERVED]: using Garmin HRV baseline only",
            );
        }

        match self.bristol_scale {
            1 | 2 => (
                0.75,
                14,
                "poop~~~~sample [ON -> VERIFIED Bristol 1-2]: slow transit — boosting warm dashi hydration, soluble root fiber & flaxseed oil",
            ),
            3 | 4 => {
                let spice_boost = if self.mucosal_calm_score >= 80 {
                    1.20
                } else {
                    1.0
                };
                (
                    spice_boost,
                    8,
                    "poop~~~~sample [ON -> VERIFIED Bristol 3-4 Zen Gold]: optimal microbiome harmony — full Hungarian paprika & sansho aromatics unlocked",
                )
            }
            5 => (
                0.65,
                12,
                "poop~~~~sample [ON -> VERIFIED Bristol 5]: gentle transit — tempering Scoville peak, adding fermented white miso & steamed lotus root",
            ),
            _ => (
                0.40,
                15,
                "poop~~~~sample [ON -> VERIFIED Bristol 6-7]: soothing gut mode — capping capsaicin, prioritizing soothing rice congee/millet körítés & ginger-chamomile infusion",
            ),
        }
    }
}

/// Source of an ingredient in the connected kitchen ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupplyOrigin {
    /// Connected Smart Fridge (chill zone, humidity drawer, expiry sensor).
    SmartFridge,
    /// Costco Bulk Supply Bridge (high-value organic staples, vacuum-sealed proteins & grains).
    CostcoBulk,
    /// Local Area Terroir / Farmers Market (picked up via Mobile/Area route).
    LocalAreaMarket,
}

/// A tracked ingredient across Smart Fridge, Costco Bulk Pantry, and Local Area Market.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryItem {
    pub name: String,
    pub origin: SupplyOrigin,
    pub quantity_grams: u32,
    pub days_until_expiry: u16,
    pub is_prebiotic_or_fermented: bool,
}

/// Mobile & Area Geolocation + Uninterrupted Time Foundation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AreaProximityContext {
    /// Current culinary terroir / neighborhood (e.g., `"Tokyo Cricket-Lane ↔ Tatabánya Hearth Bridge"`).
    pub area_name: String,
    /// Minutes until all family members converge at the home hearth.
    pub family_convergence_eta_mins: u16,
    /// Sacred uninterrupted window in minutes protected from work/phone notifications.
    pub uninterrupted_foundation_mins: u16,
    /// Whether Do-Not-Disturb Sanctuary Mode is locked across mobile devices.
    pub sanctuary_dnd_locked: bool,
}

impl AreaProximityContext {
    /// Verify whether the uninterrupted family time foundation meets the Zen threshold (>= 45 mins).
    pub fn foundation_standing(&self) -> Standing {
        if self.sanctuary_dnd_locked && self.uninterrupted_foundation_mins >= 45 {
            Standing::Verified
        } else if self.uninterrupted_foundation_mins >= 30 {
            Standing::Plausible
        } else {
            Standing::Unobserved
        }
    }
}
