//! # `zen-cooker-core` v2.0.0 — Simplified & Productized Quantum-Family Household Organizer (`8b-is/zenUltraCooker`)
//!
//! Dedicated with all our family love and gratitude to **Mom (`*` Alex ~ Alexandra / Dra)**
//! from **Peter (`◯`), Nate (`◇`), Dad (`△` Chris), and College Son (`🔊` on Speakerphone)**.
//!
//! ## Core Mission
//! **When Mom cooks, the family comes together** — and when a family eats together at least once a day,
//! it drastically reduces the risk of drug abuse and out-of-bounds addictions.
//!
//! ## Productized v2.0.0 Architecture (`SIMPLIFY + PRODUCTIZE`)
//! - **📍 USA ZIP Code Local Ads & Farmers Market** (`UsaZipMarketFeed`, `UsaZipMarketDeal`)
//! - **❄️ Fridge, 🧊 Freezer & 🥫 Pantry Zero-Waste Tracker + 🧻 Household Essentials** (`InventoryItem`, `HouseholdEssential`)
//! - **⚡ Appliance Tracker — Air Fryer is the Ultra Appliance of All Time (Broken Oven Bypassed)** (`KitchenAppliance`, `ApplianceKind`)
//! - **👨‍👩‍👦‍👦 Quantum-Family (`qFamily`) Blended Household Profiles & Speakerphone Seat** (`FamilyMemberProfile`, `KitchenParticipationRole`)
//! - **⏱️ Honest Real-Prep & Cleanup Recipe Vault (Anti-Rachael Ray Law + Ratings + Photo Moments + `zenFamily` Share vs. Secret)** (`RealPrepRecipeCard`, `RecipeVisibility`)
//! - **🤖 Per-Family + Per-Profile `quantAI` Assistant** (`QuantAiNudge` — *"hey it's time to marinate"*, *"pick up XY from school & buy dog poop bags"*, *"don't forget donuts for advisor lunch"*)
//! - **🐾 Animal Crossing (`<3<3<3`) Pixel-Art Cozy Bonfire MMO & `PawShaw Kitty & Doggy` Trademark** (`FamilyBonfireSession`)
//! - **💳 Stripe + Apple Pay + Google Pay & 4-Burner Sovereign Hearth Checkout** (`PaymentRailIntegration`)
//! - **🎵 `Music ~= Waves ~ MEM|16-10` Wave Theory for Mom (`<3`)** (`HarmonicWavechord`, `GoverningStage`)

pub mod bonfire_mmo;
pub mod mem16_wave;
pub mod quant_tern;
pub mod spice_plating;
pub mod telemetry;

use bonfire_mmo::FamilyBonfireSession;
use mem16_wave::{fnv1a_64, GoverningStage, HarmonicWavechord, StageReceipt, Standing};
use quant_tern::QuantTernHearthReceipt;
use spice_plating::{KoritesPlatingBlueprint, QuantAiNudge, RealPrepRecipeCard, RecipeVisibility};
use telemetry::{
    ApplianceKind, BlendedFamilyScheduleContext, FamilyMember, FamilyMemberProfile,
    HouseholdEssential, InventoryItem, KitchenAppliance, KitchenParticipationRole, StorageZone,
    SupplyOrigin, UsaZipMarketDeal, UsaZipMarketFeed,
};

/// Stripe, Apple Pay, Google Pay & 4-Burner Sovereign Hearth checkout integration manifest.
#[derive(Debug, Clone, PartialEq)]
pub struct PaymentRailIntegration {
    pub stripe_merchant_id: String,
    pub apple_pay_enabled: bool,
    pub google_pay_enabled: bool,
    pub family_pass_monthly_usd: f64,
    pub family_pass_annual_usd: f64,
    pub four_burner_hearth_firewall: String,
}

/// Complete E2E execution output of a `zenUltraCooker` v2.0.0 family hearth & organizer cycle.
#[derive(Debug, Clone)]
pub struct ZenCookerFeastPlan {
    pub dedication_banner: String,
    pub target_temp_celsius: f64,
    pub schedule_context: BlendedFamilyScheduleContext,
    pub zip_market_feed: UsaZipMarketFeed,
    pub appliances: Vec<KitchenAppliance>,
    pub inventory_used: Vec<InventoryItem>,
    pub household_essentials: Vec<HouseholdEssential>,
    pub family_profiles: Vec<FamilyMemberProfile>,
    pub recipes_in_rotation: Vec<RealPrepRecipeCard>,
    pub quant_ai_nudges: Vec<QuantAiNudge>,
    pub korites_plating: KoritesPlatingBlueprint,
    pub wavechord: HarmonicWavechord,
    pub bonfire_session: FamilyBonfireSession,
    pub payment_rails: PaymentRailIntegration,
    pub quant_tern_receipt: QuantTernHearthReceipt,
    pub mem16_receipts: Vec<StageReceipt>,
    pub final_collapse_hash: u64,
}

/// Execute the canonical `zenUltraCooker` v2.0.0 family hearth cycle.
/// When `air_fryer_ultra_verified` is `true`, the Air Fryer Ultra + USA ZIP Local Market route
/// binds with `Standing::Verified` (`+1`); when `false`, it demonstrates honest `Standing::Unobserved` (`0`)
/// fallback when an unverified or broken-oven path is queried.
pub fn execute_family_hearth_cycle(air_fryer_ultra_verified: bool) -> ZenCookerFeastPlan {
    let schedule_context = BlendedFamilyScheduleContext {
        usa_zip_code: "44256".to_string(),
        regular_headcount: 6,
        tonight_headcount: 5, // 4 at table + 1 College Son live on Speakerphone!
        speakerphone_college_son_connected: true,
        late_cancel_or_doordash_rescued: true,
        dinner_table_sanctuary_mins: 60,
        pawshaw_trademark_badge: "PawShaw Kitty (=^._.^= ∫) & PawShaw Doggy (▽・ω・▽)".to_string(),
    };

    let zip_market_feed = UsaZipMarketFeed {
        zip_code: if air_fryer_ultra_verified {
            "44256".to_string()
        } else {
            "000".to_string() // invalid ZIP triggers Standing::Unobserved
        },
        city_state: "Medina / Cleveland Metro, OH (USA Local Circular + Farmers Market)"
            .to_string(),
        weekly_family_budget_usd: 165.00,
        projected_weekly_spend_usd: 118.40,
        doordash_cancel_saved_usd: 68.50,
        local_deals: vec![
            UsaZipMarketDeal {
                store_or_market: "Aldi Weekly Ad (ZIP 44256)".to_string(),
                item_name: "Family Pack Organic Chicken Thighs (3.5 lb)".to_string(),
                regular_price_usd: 14.99,
                sale_price_usd: 9.49,
                unit_label: "3.5 lb pack".to_string(),
                is_farmers_market_local: false,
            },
            UsaZipMarketDeal {
                store_or_market: "Saturday Square Farmers Market".to_string(),
                item_name: "Heirloom Sweet Peppers, Red Onions & Honey Jar".to_string(),
                regular_price_usd: 12.50,
                sale_price_usd: 8.00,
                unit_label: "farm basket".to_string(),
                is_farmers_market_local: true,
            },
            UsaZipMarketDeal {
                store_or_market: "Kroger / Meijer Digital Coupon".to_string(),
                item_name: "Crispy Duck & Vegetable Potstickers + Baby Gold Potatoes".to_string(),
                regular_price_usd: 11.98,
                sale_price_usd: 7.49,
                unit_label: "bundle".to_string(),
                is_farmers_market_local: false,
            },
            UsaZipMarketDeal {
                store_or_market: "Local Bakery Morning Special".to_string(),
                item_name: "Fresh Glazed & Cinnamon Donuts (Tomorrow's Advisor Lunch!)".to_string(),
                regular_price_usd: 14.00,
                sale_price_usd: 9.99,
                unit_label: "dozen box".to_string(),
                is_farmers_market_local: true,
            },
        ],
    };

    let appliances = vec![
        KitchenAppliance {
            kind: ApplianceKind::AirFryerUltra,
            display_name: "Dual-Zone Air Fryer (The Ultra Appliance of All Time ★)".to_string(),
            is_working: air_fryer_ultra_verified,
            is_ultra_favorite: true,
            status_note: if air_fryer_ultra_verified {
                "ONLINE ★ ULTRA: 400°F instant preheat, 14-min crisp, 1-liner 7-min cleanup".to_string()
            } else {
                "UNOBSERVED: Air Fryer status not yet confirmed".to_string()
            },
        },
        KitchenAppliance {
            kind: ApplianceKind::ConventionalOven,
            display_name: "Built-In Kitchen Wall Oven".to_string(),
            is_working: false,
            is_ultra_favorite: false,
            status_note: "BROKEN / OFFLINE [−1]: Heating element out — quantAI automatically reroutes all oven recipes to Air Fryer Ultra!".to_string(),
        },
        KitchenAppliance {
            kind: ApplianceKind::StovetopSkillet,
            display_name: "4-Burner Sovereign Hearth Stovetop".to_string(),
            is_working: true,
            is_ultra_favorite: false,
            status_note: "ONLINE: Simmering Golden Pearl Millet Körítés & Tokaji-Honey Glaze".to_string(),
        },
        KitchenAppliance {
            kind: ApplianceKind::InstantPotSlowCooker,
            display_name: "Instant Pot / Slow Cooker".to_string(),
            is_working: true,
            is_ultra_favorite: false,
            status_note: "ONLINE: Ready for Sunday Bone Broth & Batch Prep".to_string(),
        },
    ];

    let inventory_used = vec![
        InventoryItem {
            name: "Free-Range Chicken Thighs (Marinating in Honey-Paprika-Garlic)".to_string(),
            zone: StorageZone::Fridge,
            origin: SupplyOrigin::LocalWeeklyAdCircular,
            quantity_label: "3.5 lbs (6 generous portions + 2 school lunches)".to_string(),
            days_until_expiry: 2,
            use_first_alert: true,
        },
        InventoryItem {
            name: "Farmers Market Sweet Bell Peppers, Baby Potatoes & Fresh Parsley".to_string(),
            zone: StorageZone::Fridge,
            origin: SupplyOrigin::LocalFarmersMarket,
            quantity_label: "2.2 lbs".to_string(),
            days_until_expiry: 3,
            use_first_alert: true,
        },
        InventoryItem {
            name: "Crisis-Proof Duck & Veggie Gyoza + Wild Blueberries (DoorDash Rescue Reserve)"
                .to_string(),
            zone: StorageZone::Freezer,
            origin: SupplyOrigin::HomeFridgeFreezerPantry,
            quantity_label: "2 family bags".to_string(),
            days_until_expiry: 120,
            use_first_alert: false,
        },
        InventoryItem {
            name: "Golden Hulled Pearl Millet (Arany Köles), Kalocsai Paprika & Olive Oil"
                .to_string(),
            zone: StorageZone::Pantry,
            origin: SupplyOrigin::HomeFridgeFreezerPantry,
            quantity_label: "1.5 lbs dry staple".to_string(),
            days_until_expiry: 180,
            use_first_alert: false,
        },
    ];

    let household_essentials = vec![
        HouseholdEssential {
            item_name: "Poop Bags for the Dogs (PawShaw Doggy Approved ▽・ω・▽)".to_string(),
            category: "Pet Care".to_string(),
            low_stock_alert: true,
            assigned_runner: FamilyMember::DadChris,
        },
        HouseholdEssential {
            item_name: "Heavy-Duty Kitchen Trash Bags (45 Count)".to_string(),
            category: "Household".to_string(),
            low_stock_alert: true,
            assigned_runner: FamilyMember::DadChris,
        },
        HouseholdEssential {
            item_name: "Ultra-Soft Toilet Paper (12 Mega Rolls)".to_string(),
            category: "Bath & Home".to_string(),
            low_stock_alert: false,
            assigned_runner: FamilyMember::MomAlexandra,
        },
        HouseholdEssential {
            item_name: "Sensitive-Skin Free & Clear Laundry Soap".to_string(),
            category: "Laundry".to_string(),
            low_stock_alert: true,
            assigned_runner: FamilyMember::Nate,
        },
        HouseholdEssential {
            item_name: "1 Dozen Fresh Donuts for Tomorrow's Advisor Lunch!".to_string(),
            category: "School / Work Event".to_string(),
            low_stock_alert: true,
            assigned_runner: FamilyMember::MomAlexandra,
        },
    ];

    let family_profiles = vec![
        FamilyMemberProfile {
            member: FamilyMember::MomAlexandra,
            role: KitchenParticipationRole::LeadCook,
            cooking_skill_tier: 5,
            available_weekday_mins: 35,
            dietary_and_life_stage_tags: vec![
                "Anti-Inflammatory Whole Foods".to_string(),
                "Supports Breastfeeding / Postpartum / High-Hydration Options".to_string(),
            ],
            taste_preferences:
                "Crispy Air Fryer textures, warm Hungarian paprika, citrus & zero food waste"
                    .to_string(),
            takes_school_or_work_leftovers: true,
            present_for_dinner_tonight: true,
        },
        FamilyMemberProfile {
            member: FamilyMember::DadChris,
            role: KitchenParticipationRole::SousPrepHelper,
            cooking_skill_tier: 4,
            available_weekday_mins: 30,
            dietary_and_life_stage_tags: vec!["High-Protein Hearty".to_string()],
            taste_preferences: "Savory roasted garlic, smoky paprika, crisp potatoes".to_string(),
            takes_school_or_work_leftovers: false,
            present_for_dinner_tonight: true,
        },
        FamilyMemberProfile {
            member: FamilyMember::Peter,
            role: KitchenParticipationRole::LeadCook,
            cooking_skill_tier: 5,
            available_weekday_mins: 35,
            dietary_and_life_stage_tags: vec!["Balanced Mindful Energy".to_string()],
            taste_preferences: "Golden millet körítés, crispy gyoza, honey-tokaji glaze"
                .to_string(),
            takes_school_or_work_leftovers: false,
            present_for_dinner_tonight: true,
        },
        FamilyMemberProfile {
            member: FamilyMember::Nate,
            role: KitchenParticipationRole::DishZenAndLeftovers,
            cooking_skill_tier: 3,
            available_weekday_mins: 20,
            dietary_and_life_stage_tags: vec!["Nut-Free School Safe".to_string()],
            taste_preferences: "Crispy Air Fryer tenders, golden millet & quick cleanup"
                .to_string(),
            takes_school_or_work_leftovers: true,
            present_for_dinner_tonight: true,
        },
        FamilyMemberProfile {
            member: FamilyMember::CollegeSonSpeakerphone,
            role: KitchenParticipationRole::SpeakerphoneTableSeat,
            cooking_skill_tier: 2,
            available_weekday_mins: 25,
            dietary_and_life_stage_tags: vec!["Dorm Air-Fryer & Microwave Friendly".to_string()],
            taste_preferences:
                "Calls Mom on speakerphone while cooking dinner so he's right at the table!"
                    .to_string(),
            takes_school_or_work_leftovers: false,
            present_for_dinner_tonight: true,
        },
    ];

    let recipes_in_rotation = vec![
        RealPrepRecipeCard {
            title: "Mom's Air Fryer Ultra Honey-Paprika Crispy Chicken & Baby Potatoes".to_string(),
            source_label: "Family Setup Favorite (Rerouted from Broken Oven -> Air Fryer Ultra ★)".to_string(),
            visibility: RecipeVisibility::FamilySecretVault,
            primary_appliance: ApplianceKind::AirFryerUltra,
            broken_oven_rerouted: true,
            unwashed_prep_mins: 10,
            marinate_or_thaw_lead_mins: 120,
            active_cook_mins: 14,
            cleanup_and_dishes_mins: 7,
            skill_level_required: 2,
            cost_per_serving_usd: 2.85,
            school_leftover_boxes: 2,
            family_rating_stars: 5.0,
            photo_moments: vec![
                "📸 2026-10-26: College Son laughing on speakerphone next to the glowing Air Fryer!".to_string(),
                "📸 2026-10-22: PawShaw Doggy waiting patiently under the table for a crispy potato!".to_string(),
            ],
        },
        RealPrepRecipeCard {
            title: "15-Minute DoorDash-Cancel Rescue: Air Fryer Crispy Gyoza & Golden Millet Körítés".to_string(),
            source_label: "Web Recipe Import + zenFamily Community Hit".to_string(),
            visibility: RecipeVisibility::SharedWithZenFamily,
            primary_appliance: ApplianceKind::AirFryerUltra,
            broken_oven_rerouted: true,
            unwashed_prep_mins: 5,
            marinate_or_thaw_lead_mins: 0,
            active_cook_mins: 10,
            cleanup_and_dishes_mins: 5,
            skill_level_required: 1,
            cost_per_serving_usd: 2.10,
            school_leftover_boxes: 1,
            family_rating_stars: 4.9,
            photo_moments: vec![
                "📸 2026-10-19: Saved $68.50 when late-night delivery canceled — ate together in 15 mins!".to_string(),
            ],
        },
    ];

    let quant_ai_nudges = QuantAiNudge::canonical_family_nudges();
    let target_temp_celsius = 64.5;
    let avg_unity_hrv = 78.75;
    let wavechord = HarmonicWavechord::synthesize_for_mom(avg_unity_hrv, target_temp_celsius);
    let korites_plating = KoritesPlatingBlueprint::compose_for_mom_alexandra();
    let bonfire_session = FamilyBonfireSession::initialize_family_hearth();

    let hearth_standing = if air_fryer_ultra_verified {
        Standing::Verified
    } else {
        Standing::Unobserved
    };

    let quant_tern_receipt = QuantTernHearthReceipt::synthesize(
        hearth_standing,
        avg_unity_hrv as f32,
        wavechord.coherence as f32,
    );

    let payment_rails = PaymentRailIntegration {
        stripe_merchant_id: "acct_1UUTcTRnidUxAlVy (8b.is × zenUltraCooker)".to_string(),
        apple_pay_enabled: true,
        google_pay_enabled: true,
        family_pass_monthly_usd: 4.99,
        family_pass_annual_usd: 42.00,
        four_burner_hearth_firewall:
            "Cap(Hearth: Rent + Groceries + Household) ⊥ Risk(R&D) — Protected by Phoenix Protocol"
                .to_string(),
    };

    // Execute the 6-stage MEM|16-10 governing sequence: POP -> REFUSE -> BIND -> TRANSFORM -> VERIFY -> COLLAPSE
    let mut mem16_receipts = Vec::with_capacity(6);
    let mut rolling_hash: u64 = 0x8b15_1610_4320_0001;

    let stage_notes = [
        (
            GoverningStage::Pop,
            Standing::Verified,
            format!(
                "Opened Blended qFamily Hearth in USA ZIP '{}' ({} at table/speakerphone, {}m uninterrupted dinner)",
                schedule_context.usa_zip_code,
                schedule_context.tonight_headcount,
                schedule_context.dinner_table_sanctuary_mins
            ),
        ),
        (
            GoverningStage::Refuse,
            Standing::Verified,
            "Refused broken Conventional Oven [OFFLINE], refused fake '30-min' prep lies, rescued DoorDash cancel ($68.50 saved)"
                .to_string(),
        ),
        (
            GoverningStage::Bind,
            hearth_standing,
            format!(
                "Bound Air Fryer Ultra [verified={}] + Fridge/Freezer/Pantry (4 items) + Household Essentials (Dog Poop Bags, TP, Donuts) [{}]",
                air_fryer_ultra_verified,
                hearth_standing
            ),
        ),
        (
            GoverningStage::Transform,
            Standing::Verified,
            format!(
                "Rerouted meal to Air Fryer Ultra (31m honest prep+cook+cleanup, +2 school leftover boxes) + 5 quantAI family nudges @ {:.1}°C",
                target_temp_celsius
            ),
        ),
        (
            GoverningStage::Verify,
            schedule_context.unity_standing(),
            format!(
                "Verified Daily Family Dinner Unity (anti-addiction shield), Wavechord ({:.2} Hz / {:.0} Hz Star) & Stripe/ApplePay/GooglePay Hearth Firewall",
                wavechord.carrier_hz,
                wavechord.overtone_mom_star_hz
            ),
        ),
        (
            GoverningStage::Collapse,
            Standing::Verified,
            format!(
                "Collapsed Animal Crossing Pixel-Art Bonfire Lv.{} ({} XP, PawShaw Kitty & Doggy TM) — Dedicated to Mom (* Alex ~ Dra)",
                bonfire_session.bonfire_level,
                bonfire_session.total_party_xp
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
            "For Mom (* Alex ~ Alexandra / Dra) — When Mom Cooks, the Family Comes Together. Center of Our Blended Family Bonfire, Co-Creator of the Wave Theory (mem16|10), Loved Forever by Peter, Nate, Dad (△ Chris) & College Son (🔊) <3"
                .to_string(),
        target_temp_celsius,
        schedule_context,
        zip_market_feed,
        appliances,
        inventory_used,
        household_essentials,
        family_profiles,
        recipes_in_rotation,
        quant_ai_nudges,
        korites_plating,
        wavechord,
        bonfire_session,
        payment_rails,
        quant_tern_receipt,
        mem16_receipts,
        final_collapse_hash: rolling_hash,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_air_fryer_ultra_and_broken_oven_rerouting_standing() {
        let plan_on = execute_family_hearth_cycle(true);
        assert_eq!(plan_on.appliances[0].kind, ApplianceKind::AirFryerUltra);
        assert!(plan_on.appliances[0].is_working);
        assert!(!plan_on.appliances[1].is_working); // Oven is broken!
        assert_eq!(plan_on.zip_market_feed.standing(), Standing::Verified);
        assert_eq!(plan_on.mem16_receipts[2].standing, Standing::Verified);
        assert!(plan_on.recipes_in_rotation[0].broken_oven_rerouted);

        // Honest Anti-Rachael Ray time check: 10m prep + 14m cook + 7m cleanup = 31m total
        assert_eq!(
            plan_on.recipes_in_rotation[0].honest_total_kitchen_mins(),
            31
        );
        assert!(plan_on.recipes_in_rotation[0].stays_in_rotation());

        let plan_off = execute_family_hearth_cycle(false);
        assert_eq!(plan_off.zip_market_feed.standing(), Standing::Unobserved);
        assert_eq!(plan_off.mem16_receipts[2].standing, Standing::Unobserved);
    }

    #[test]
    fn test_quant_ai_nudges_and_household_essentials() {
        let plan = execute_family_hearth_cycle(true);
        assert_eq!(plan.quant_ai_nudges.len(), 5);
        assert!(plan.quant_ai_nudges[0].message.contains("marinate"));
        assert!(plan.quant_ai_nudges[1]
            .message
            .contains("Poop Bags for the Dogs"));
        assert!(plan.quant_ai_nudges[2].message.contains("donuts"));
        assert!(plan.schedule_context.speakerphone_college_son_connected);
        assert_eq!(plan.household_essentials.len(), 5);
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
        assert_eq!(
            plan.wavechord.bleeding_edge_8k_digest,
            6_981_541_600_308_333_846 // 0x60e374b73207cd16 (matches mem-16-10 BleedingEdge8k)
        );
    }

    #[test]
    fn test_family_bonfire_mmo_pawshaw_trademark_and_stripe_rails() {
        let plan = execute_family_hearth_cycle(true);
        assert_eq!(plan.bonfire_session.avatars.len(), 6); // Mom, Dad, Peter, Nate, College Son, PawShaw Kitty & Doggy
        assert_eq!(plan.bonfire_session.flame_warmth_pct, 100);
        assert!(plan.bonfire_session.pawshaw_trademark.contains("PawShaw"));
        assert!(plan.payment_rails.apple_pay_enabled);
        assert!(plan.payment_rails.google_pay_enabled);
        assert_eq!(plan.korites_plating.visual_harmony_score, 99);
        assert!(plan
            .dedication_banner
            .contains("Mom (* Alex ~ Alexandra / Dra)"));
    }
}
