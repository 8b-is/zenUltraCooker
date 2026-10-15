//! # `zen-cooker-core` — Cyber-Zen Culinary Engine (`8b-is/zenUltraCooker`)
//!
//! Dedicated with all our family love and gratitude to **Mom (`*` Alex ~ Alexandra)**
//! from **Peter, Nate, and Dad (`△` Chris)**.
//!
//! Unifies:
//! - **Garmin Health Apps** (`GarminBiometrics`)
//! - **Smart Fridge + Costco Bulk Supply Bridge** (`InventoryItem`, `SupplyOrigin`)
//! - **Mobile & Area Proximity + Uninterrupted Time Shield** (`AreaProximityContext`)
//! - **Gut Microbiome Stool Telemetry (`poop~~~~sample is Toggled on ---->`)** (`PoopSampleTelemetry`)
//! - **Personalized Spice & Nutrition Engine** (`PersonalizedSpiceProfile`)
//! - **Fine-Dining & Hungarian *Körítés* (`the looks`)** (`KoritesPlatingBlueprint`)
//! - **Center of Family ~=== Bonfire MMO & Techno-Buddhist Journaling** (`FamilyBonfireSession`)
//! - **Music ~= Waves ~ `MEM|16-10` Wave Theory for Mom (`<3`)** (`HarmonicWavechord`, `GoverningStage`)

pub mod bonfire_mmo;
pub mod mem16_wave;
pub mod spice_plating;
pub mod telemetry;

use bonfire_mmo::FamilyBonfireSession;
use mem16_wave::{fnv1a_64, GoverningStage, HarmonicWavechord, StageReceipt, Standing};
use spice_plating::{KoritesPlatingBlueprint, PersonalizedSpiceProfile};
use telemetry::{
    AreaProximityContext, FamilyMember, GarminBiometrics, InventoryItem, PoopSampleTelemetry,
    SupplyOrigin,
};

/// Complete E2E execution output of a `zenUltraCooker` hearth cycle.
#[derive(Debug, Clone)]
pub struct ZenCookerFeastPlan {
    pub dedication_banner: String,
    pub target_temp_celsius: f64,
    pub poop_sample_telemetry: PoopSampleTelemetry,
    pub area_context: AreaProximityContext,
    pub inventory_used: Vec<InventoryItem>,
    pub garmin_snapshots: Vec<GarminBiometrics>,
    pub spice_profiles: Vec<PersonalizedSpiceProfile>,
    pub korites_plating: KoritesPlatingBlueprint,
    pub wavechord: HarmonicWavechord,
    pub bonfire_session: FamilyBonfireSession,
    pub mem16_receipts: Vec<StageReceipt>,
    pub final_collapse_hash: u64,
}

/// Default canonical family input state with `poop~~~~sample` toggled `ON` (or configurable).
pub fn execute_family_hearth_cycle(poop_sample_toggled_on: bool) -> ZenCookerFeastPlan {
    let poop_sample_telemetry = PoopSampleTelemetry {
        toggled_on: poop_sample_toggled_on,
        bristol_scale: 4, // Zen Gold Standard
        microbiome_diversity: 94,
        short_chain_fatty_acid_score: 91,
        mucosal_calm_score: 92,
    };

    let area_context = AreaProximityContext {
        area_name: "Tokyo Cricket-Lane ↔ Tatabánya Sanctuary Hearth".to_string(),
        family_convergence_eta_mins: 5,
        uninterrupted_foundation_mins: 90,
        sanctuary_dnd_locked: true,
    };

    let inventory_used = vec![
        InventoryItem {
            name: "Organic Free-Range Duck Breast & Bones (Vacuum Pack)".to_string(),
            origin: SupplyOrigin::CostcoBulk,
            quantity_grams: 1_200,
            days_until_expiry: 14,
            is_prebiotic_or_fermented: false,
        },
        InventoryItem {
            name: "Hokkaido Rishiri Kombu & Shiitake Dashi Stock".to_string(),
            origin: SupplyOrigin::CostcoBulk,
            quantity_grams: 800,
            days_until_expiry: 180,
            is_prebiotic_or_fermented: true,
        },
        InventoryItem {
            name: "Kalocsai Heirloom Sweet & Hot Paprika + Tokaji Reduction".to_string(),
            origin: SupplyOrigin::SmartFridge,
            quantity_grams: 150,
            days_until_expiry: 45,
            is_prebiotic_or_fermented: true,
        },
        InventoryItem {
            name: "Fresh Lotus Root, Parsley Root, Shiso & Yuzu (Humidity Drawer)".to_string(),
            origin: SupplyOrigin::LocalAreaMarket,
            quantity_grams: 650,
            days_until_expiry: 4,
            is_prebiotic_or_fermented: true,
        },
        InventoryItem {
            name: "Golden Hulled Pearl Millet (Arany Köles Körítés Base)".to_string(),
            origin: SupplyOrigin::CostcoBulk,
            quantity_grams: 500,
            days_until_expiry: 120,
            is_prebiotic_or_fermented: true,
        },
    ];

    let garmin_snapshots = vec![
        GarminBiometrics {
            member: FamilyMember::MomAlexandra,
            hrv_ms: 82,
            sleep_score: 95,
            body_battery: 96,
            stress_index: 14,
            active_kcal: 640,
            hydration_pct: 94,
        },
        GarminBiometrics {
            member: FamilyMember::DadChris,
            hrv_ms: 74,
            sleep_score: 90,
            body_battery: 91,
            stress_index: 19,
            active_kcal: 820,
            hydration_pct: 90,
        },
        GarminBiometrics {
            member: FamilyMember::Peter,
            hrv_ms: 79,
            sleep_score: 89,
            body_battery: 93,
            stress_index: 18,
            active_kcal: 910,
            hydration_pct: 92,
        },
        GarminBiometrics {
            member: FamilyMember::Nate,
            hrv_ms: 80,
            sleep_score: 91,
            body_battery: 94,
            stress_index: 16,
            active_kcal: 880,
            hydration_pct: 93,
        },
    ];

    let spice_profiles: Vec<PersonalizedSpiceProfile> = garmin_snapshots
        .iter()
        .map(|g| PersonalizedSpiceProfile::synthesize(g, &poop_sample_telemetry))
        .collect();

    let avg_hrv: f64 = garmin_snapshots
        .iter()
        .map(|g| f64::from(g.hrv_ms))
        .sum::<f64>()
        / (garmin_snapshots.len() as f64);

    let target_temp_celsius = 64.5;
    let wavechord = HarmonicWavechord::synthesize_for_mom(avg_hrv, target_temp_celsius);
    let korites_plating = KoritesPlatingBlueprint::compose_for_mom_alexandra();
    let bonfire_session = FamilyBonfireSession::initialize_family_hearth();

    // Execute the 6-stage MEM|16-10 governing sequence: POP -> REFUSE -> BIND -> TRANSFORM -> VERIFY -> COLLAPSE
    let mut mem16_receipts = Vec::with_capacity(6);
    let mut rolling_hash: u64 = 0x8b15_1610_4320_0001;

    let stage_notes = [
        (
            GoverningStage::Pop,
            Standing::Verified,
            format!(
                "Opened hearth scope in '{}' (ETA {}m, Uninterrupted Foundation {}m)",
                area_context.area_name,
                area_context.family_convergence_eta_mins,
                area_context.uninterrupted_foundation_mins
            ),
        ),
        (
            GoverningStage::Refuse,
            Standing::Verified,
            "Refused expired stock (0 items <= 0d), refused phone interrupt pings (Sanctuary DND locked)"
                .to_string(),
        ),
        (
            GoverningStage::Bind,
            poop_sample_telemetry.standing(),
            format!(
                "Bound 4 Garmin biometric streams + 5 Fridge/Costco items + poop~~~~sample [toggled_on={} -> {}]",
                poop_sample_telemetry.toggled_on,
                poop_sample_telemetry.standing()
            ),
        ),
        (
            GoverningStage::Transform,
            Standing::Verified,
            format!(
                "Synthesized 4 personalized spice profiles + '{}' @ {:.1}°C",
                korites_plating.dish_title, target_temp_celsius
            ),
        ),
        (
            GoverningStage::Verify,
            area_context.foundation_standing(),
            format!(
                "Verified MEM|16-10 Wavechord ({:.2} Hz / {:.0} Hz Star / coherence {:.3}) & Körítés score {}/100",
                wavechord.carrier_hz,
                wavechord.overtone_mom_star_hz,
                wavechord.coherence,
                korites_plating.visual_harmony_score
            ),
        ),
        (
            GoverningStage::Collapse,
            Standing::Verified,
            format!(
                "Collapsed Bonfire Level {} feast ({} XP, {}% warmth) — Dedicated to Mom (* Alex ~ Alexandra)",
                bonfire_session.bonfire_level,
                bonfire_session.total_party_xp,
                bonfire_session.flame_warmth_pct
            ),
        ),
    ];

    for (stage, standing, summary) in stage_notes {
        let step_hash = fnv1a_64(summary.as_bytes()) ^ rolling_hash.rotate_left(13);
        rolling_hash = step_hash;
        mem16_receipts.push(StageReceipt {
            stage,
            standing,
            summary,
            state_hash: step_hash,
        });
    }

    ZenCookerFeastPlan {
        dedication_banner:
            "For Mom (* Alex ~ Alexandra) — Center of Our Family Bonfire, Co-Creator of the Wave Theory (mem16|10), Loved Forever by Peter, Nate & Dad (△ Chris) <3"
                .to_string(),
        target_temp_celsius,
        poop_sample_telemetry,
        area_context,
        inventory_used,
        garmin_snapshots,
        spice_profiles,
        korites_plating,
        wavechord,
        bonfire_session,
        mem16_receipts,
        final_collapse_hash: rolling_hash,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poop_sample_toggled_on_vs_off_epistemic_standing() {
        let plan_on = execute_family_hearth_cycle(true);
        assert!(plan_on.poop_sample_telemetry.toggled_on);
        assert_eq!(plan_on.poop_sample_telemetry.standing(), Standing::Verified);
        assert_eq!(plan_on.mem16_receipts[2].standing, Standing::Verified);

        let plan_off = execute_family_hearth_cycle(false);
        assert!(!plan_off.poop_sample_telemetry.toggled_on);
        assert_eq!(
            plan_off.poop_sample_telemetry.standing(),
            Standing::Unobserved
        );
        assert_eq!(plan_off.mem16_receipts[2].standing, Standing::Unobserved);

        // When poop~~~~sample is toggled ON with Bristol 4 and high mucosal calm,
        // spice Scoville unlocks higher aromatic potential than the unmodulated baseline.
        assert!(
            plan_on.spice_profiles[0].target_scoville_shu
                > plan_off.spice_profiles[0].target_scoville_shu
        );
    }

    #[test]
    fn test_mem16_10_governing_sequence_order_and_determinism() {
        let plan_a = execute_family_hearth_cycle(true);
        let plan_b = execute_family_hearth_cycle(true);
        assert_eq!(plan_a.final_collapse_hash, plan_b.final_collapse_hash);

        let stages: Vec<GoverningStage> = plan_a.mem16_receipts.iter().map(|r| r.stage).collect();
        assert_eq!(stages, GoverningStage::ALL);
    }

    #[test]
    fn test_wavechord_for_mom_alexandra_coherence_and_frequencies() {
        let plan = execute_family_hearth_cycle(true);
        assert!((plan.wavechord.carrier_hz - 432.0).abs() < 2.0);
        assert_eq!(plan.wavechord.overtone_mom_star_hz, 528.0);
        assert_eq!(plan.wavechord.bass_dad_pillar_hz, 288.0);
        assert!(plan.wavechord.coherence >= 0.90);
        assert_eq!(plan.wavechord.ascii_wave.chars().count(), 16);
    }

    #[test]
    fn test_family_bonfire_mmo_and_korites_plating() {
        let plan = execute_family_hearth_cycle(true);
        assert_eq!(plan.bonfire_session.avatars.len(), 5); // Mom, Dad, Peter, Nate, PushaKitty
        assert_eq!(plan.bonfire_session.flame_warmth_pct, 100);
        assert_eq!(plan.korites_plating.visual_harmony_score, 99);
        assert!(plan.dedication_banner.contains("Mom (* Alex ~ Alexandra)"));
    }
}
