//! # Personalized Spice Engine & Fine-Dining *Körítés* (The Looks)
//!
//! Combines Garmin biometrics + toggled `poop~~~~sample` microbiome telemetry + individual
//! palate profiles to synthesize:
//! 1. **Per-Person Personalized Spice Dosing** (`PersonalizedSpiceProfile`) — Scoville Heat Units (SHU),
//!    Kalocsai/Szegedi Hungarian paprika blend, Japanese sansho/yuzu zest, and prebiotic fiber grams.
//! 2. **Fine-Dining & *Körítés* Plating Spec** (`KoritesPlatingBlueprint`) — Elevating home cooking
//!    into Michelin / Kaiseki visual poetry (`korítés - the looks`) with golden-ratio geometry,
//!    color contrast harmony, and Mom's 8-pointed star (`*`) gold-leaf garnish.

use crate::telemetry::{FamilyMember, GarminBiometrics, PoopSampleTelemetry};

/// Individualized spice, herb, and prebiotic profile for a family member.
#[derive(Debug, Clone, PartialEq)]
pub struct PersonalizedSpiceProfile {
    pub member: FamilyMember,
    /// Target Scoville Heat Units (SHU) tailored to recovery & gut state.
    pub target_scoville_shu: u32,
    /// Sweet noble Hungarian paprika (*Édesnemes*) in grams.
    pub sweet_paprika_grams: f64,
    /// Hot artisanal paprika (*Erős Pista / Kalocsai Csípős*) in grams.
    pub hot_paprika_grams: f64,
    /// Japanese Yuzu-Sansho & toasted sesame aromatic notes in grams.
    pub yuzu_sansho_grams: f64,
    /// Soluble prebiotic fiber & fermented dashi reduction in grams.
    pub prebiotic_boost_grams: u8,
    /// Human-readable culinary note explaining why this profile was selected.
    pub rationale: String,
}

impl PersonalizedSpiceProfile {
    /// Compute personalized spice dosing from Garmin biometrics and the `poop~~~~sample` toggle.
    pub fn synthesize(garmin: &GarminBiometrics, poop: &PoopSampleTelemetry) -> Self {
        let base_shu: u32 = match garmin.member {
            FamilyMember::MomAlexandra => 1_800, // Refined, luminous warmth with floral yuzu & noble sweet paprika
            FamilyMember::DadChris => 4_500,     // Deep Magyar robust warmth & smoky oak notes
            FamilyMember::Peter => 6_200,        // Cyber-zen Tokyo-Tatabánya fusion heat
            FamilyMember::Nate => 5_400,         // Galactic vibrant spice & crisp herbal finish
        };

        let recovery = garmin.recovery_index();
        let (gut_mult, prebiotic_grams, gut_note) = poop.digestive_modulation();

        // Scale Scoville gently by both Garmin recovery and gut mucosal modulation
        let combined_mult = (0.65 + recovery * 0.35) * gut_mult;
        let target_scoville_shu =
            ((f64::from(base_shu) * combined_mult).round() as u32).clamp(300, 12_000);

        let sweet_paprika_grams = match garmin.member {
            FamilyMember::MomAlexandra => 4.5,
            FamilyMember::DadChris => 5.0,
            FamilyMember::Peter => 4.2,
            FamilyMember::Nate => 4.2,
        };

        let hot_paprika_grams =
            ((f64::from(target_scoville_shu) / 3_000.0) * 100.0).round() / 100.0;
        let yuzu_sansho_grams = ((1.2 + recovery * 0.8) * 100.0).round() / 100.0;

        let rationale = format!(
            "{} | Garmin Recovery {:.0}% (HRV {}ms, Battery {}%) | {}",
            garmin.member.display_name(),
            recovery * 100.0,
            garmin.hrv_ms,
            garmin.body_battery,
            gut_note
        );

        Self {
            member: garmin.member,
            target_scoville_shu,
            sweet_paprika_grams,
            hot_paprika_grams,
            yuzu_sansho_grams,
            prebiotic_boost_grams: prebiotic_grams,
            rationale,
        }
    }
}

/// Fine-Dining & *Körítés* (`korítés - the looks`) visual plating specification.
#[derive(Debug, Clone, PartialEq)]
pub struct KoritesPlatingBlueprint {
    /// Signature dish title honoring Mom (`*` Alexandra).
    pub dish_title: String,
    /// Primary centerpiece protein/broth base.
    pub centerpiece: String,
    /// Hungarian *Körítés* (artisan side & garnish architecture).
    pub hungarian_korites_layers: Vec<String>,
    /// Japanese Kaiseki visual & negative-space (*Ma*) touches.
    pub kaiseki_visual_accents: Vec<String>,
    /// Ceramic vessel & lighting specification.
    pub vessel_and_lighting: String,
    /// Aesthetic harmony score (`0..=100`).
    pub visual_harmony_score: u8,
}

impl KoritesPlatingBlueprint {
    /// Compose the signature Michelin/Kaiseki *Körítés* blueprint for the family bonfire feast.
    pub fn compose_for_mom_alexandra() -> Self {
        Self {
            dish_title: "Csillag-Hullám Kaiseki Halászlé & Duck Confit Gyoza with Golden Millet Körítés (Dedicated to Mom * Alexandra)".to_string(),
            centerpiece: "48-Hour Precision Sous-Vide Tokaji-Glazed Duck & Silky Kombu-Paprika Consommé (64.5°C Zen Thermal Hold)".to_string(),
            hungarian_korites_layers: vec![
                "Saffron & toasted-butter pearl millet (arany köles körítés) shaped in a sacred Fibonacci spiral".to_string(),
                "Charred Kalocsai heirloom sweet pepper ribbons & pickled purple pearl onions".to_string(),
                "Crisp parsley-root pavé brushed with fermented black-garlic & plum (szilva) glaze".to_string(),
            ],
            kaiseki_visual_accents: vec![
                "24k edible gold-leaf 8-pointed Star (*) crowning the center in honor of Mom (Alexandra)".to_string(),
                "Crisp lotus-root geometric triangle (△) chip honoring Dad (Chris)".to_string(),
                "Micro-shiso, sakura blossom salt rim, and 38% open porcelain negative space (Ma / 間)".to_string(),
            ],
            vessel_and_lighting: "Hand-thrown Kuro-Raku black ceramic plate with Kintsugi gold seam under 2700K warm bonfire hearth spot".to_string(),
            visual_harmony_score: 99,
        }
    }
}
