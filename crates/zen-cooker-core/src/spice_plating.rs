//! # Real-Prep Recipe Vault, `quantAI` Family Assistant & *Körítés* Presentation
//!
//! Productizes `zenUltraCooker` v2.0.0 around Dra (Alexandra) and Péter's real-life family laws:
//! 1. **The Anti-Rachael Ray Honest Prep & Cleanup Law (`RealPrepRecipeCard`)**:
//!    - *"Prep times need to be real, not Rachel Ray's 30 minute meals that do not include you did
//!      all this prep 2 days ago when you came home from the store. Cooking level and skill and time
//!      that you have which includes cleanup."*
//!    - Tracks `unwashed_prep_mins + active_cook_mins + cleanup_and_dishes_mins` plus advance
//!      `marinate_or_thaw_lead_mins`.
//! 2. **Web Import + Family Favorites + `zenFamily` Share vs. Family Secret (`RecipeVisibility`)**:
//!    - Families can import recipes from the web or enter family setup favorites, choosing to share
//!      with the global `zenFamily` community (`+1`) or keep them as **Family Secrets (`🔒`)**.
//!    - Family members rate meals (`1..=5 ★`) and attach **Photo Moments** to keep top meals in rotation.
//! 3. **Per-Family + Per-Profile `quantAI` Assistant (`QuantAiNudge`)**:
//!    - Learns household patterns and sends warm, actionable nudges:
//!      - *"Hey it's time to marinate!"*
//!      - *"Hey please pick up XY from school or buy poop bags for the dogs!"*
//!      - *"Hey, you have snacks tomorrow for advisor lunch ... don't forget to get the donuts!"*
//! 4. **Fine-Dining & Hungarian *Körítés* (`KoritesPlatingBlueprint`)**:
//!    - Honors Mom (`*` Alexandra / Dra) and Dad (`△` Chris) with effortless family-style plating.

use crate::telemetry::{ApplianceKind, FamilyMember};

/// Whether a family recipe is shared with the global `zenFamily` or kept as a Family Secret.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecipeVisibility {
    /// Shared openly with the global `zenFamily` community (`+1`).
    SharedWithZenFamily,
    /// Kept strictly inside the private Family Secret Vault (`🔒`).
    FamilySecretVault,
}

impl RecipeVisibility {
    pub fn badge(&self) -> &'static str {
        match self {
            Self::SharedWithZenFamily => "Shared with zenFamily 🌐",
            Self::FamilySecretVault => "Family Secret Vault 🔒",
        }
    }
}

/// Honest time breakdown, appliance routing, rating, and photo moments for a family recipe.
#[derive(Debug, Clone, PartialEq)]
pub struct RealPrepRecipeCard {
    pub title: String,
    pub source_label: String,
    pub visibility: RecipeVisibility,
    pub primary_appliance: ApplianceKind,
    pub broken_oven_rerouted: bool,
    /// Real raw-ingredient wash & chop time (no hidden "prep from 2 days ago"!).
    pub unwashed_prep_mins: u16,
    /// Advance marinate or freezer-thaw lead notice in minutes (`quantAI` triggers alert).
    pub marinate_or_thaw_lead_mins: u16,
    /// Active cooking time on the Air Fryer / Stovetop.
    pub active_cook_mins: u16,
    /// Honest dishwashing, Air Fryer basket soak, and counter wipe cleanup time.
    pub cleanup_and_dishes_mins: u16,
    /// Required skill level (`1..=5`).
    pub skill_level_required: u8,
    /// Estimated cost per serving in USD using local ZIP weekly ad prices.
    pub cost_per_serving_usd: f64,
    /// Number of school/work lunch leftover boxes automatically portioned.
    pub school_leftover_boxes: u8,
    /// Average family star rating (`0.0 ..= 5.0`) — `>= 4.0` stays in active weekly rotation.
    pub family_rating_stars: f32,
    /// Captured family photo moments & memories attached to this meal.
    pub photo_moments: Vec<String>,
}

impl RealPrepRecipeCard {
    /// Honest Hands-On + Cook + Cleanup total time in minutes (Anti-Rachael Ray Law).
    pub fn honest_total_kitchen_mins(&self) -> u16 {
        self.unwashed_prep_mins + self.active_cook_mins + self.cleanup_and_dishes_mins
    }

    /// Whether the family rating keeps this recipe in active rotation (`>= 4.0` stars).
    pub fn stays_in_rotation(&self) -> bool {
        self.family_rating_stars >= 4.0
    }
}

/// Proactive notification from the family's personalized `quantAI` companion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantAiNudge {
    pub target_member: FamilyMember,
    pub trigger_time_label: String,
    pub category_tag: String,
    pub message: String,
}

impl QuantAiNudge {
    /// Generate the canonical daily pattern-learned `quantAI` nudges for the blended family.
    pub fn canonical_family_nudges() -> Vec<Self> {
        vec![
            Self {
                target_member: FamilyMember::MomAlexandra,
                trigger_time_label: "4:15 PM (Marinate Window)".to_string(),
                category_tag: "MARINATE_ALERT".to_string(),
                message: "Hey Mom (* Dra), it's time to marinate the Crispy Honey-Paprika Chicken Thighs for the Air Fryer (real prep: 10m, zero oven needed!).".to_string(),
            },
            Self {
                target_member: FamilyMember::DadChris,
                trigger_time_label: "3:20 PM (School & Errand Run)".to_string(),
                category_tag: "PICKUP_AND_DOGS".to_string(),
                message: "Hey Chris (△), please pick up XY from school at 3:30 PM and buy Poop Bags for the Dogs + Trash Bags at Aldi on the way home!".to_string(),
            },
            Self {
                target_member: FamilyMember::MomAlexandra,
                trigger_time_label: "7:30 PM (Tomorrow Pattern Learned)".to_string(),
                category_tag: "ADVISOR_LUNCH_DONUTS".to_string(),
                message: "Hey Dra (*), you have snacks tomorrow for Advisor Lunch — don't forget to get the donuts!".to_string(),
            },
            Self {
                target_member: FamilyMember::CollegeSonSpeakerphone,
                trigger_time_label: "6:10 PM (Speakerphone Seat)".to_string(),
                category_tag: "SPEAKERPHONE_TABLE".to_string(),
                message: "Hey College Son (🔊), Air Fryer basket goes in now! Call Mom on speakerphone in 12 mins so you're right here around the stove with us.".to_string(),
            },
            Self {
                target_member: FamilyMember::Nate,
                trigger_time_label: "6:50 PM (Dish-Zen & Leftovers)".to_string(),
                category_tag: "CLEANUP_AND_SCHOOL_BOX".to_string(),
                message: "Hey Nate (◇), 1-basket Air Fryer liner cleanup is only 7 mins tonight + pack 2 Golden Millet & Crispy Chicken boxes for tomorrow's school lunch!".to_string(),
            },
        ]
    }
}

/// Fine-Dining & *Körítés* (`korítés - the looks`) visual plating specification.
#[derive(Debug, Clone, PartialEq)]
pub struct KoritesPlatingBlueprint {
    /// Signature dish title honoring Mom (`*` Alexandra / Dra).
    pub dish_title: String,
    /// Primary centerpiece protein/broth base (Air Fryer Ultra routed!).
    pub centerpiece: String,
    /// Hungarian *Körítés* (artisan side & garnish architecture).
    pub hungarian_korites_layers: Vec<String>,
    /// Japanese Kaiseki & PawShaw Kitty/Doggy visual touches.
    pub kaiseki_visual_accents: Vec<String>,
    /// Ceramic vessel & lighting specification.
    pub vessel_and_lighting: String,
    /// Aesthetic harmony score (`0..=100`).
    pub visual_harmony_score: u8,
}

impl KoritesPlatingBlueprint {
    /// Compose the signature Air Fryer Ultra + *Körítés* blueprint for the family bonfire feast.
    pub fn compose_for_mom_alexandra() -> Self {
        Self {
            dish_title: "Air Fryer Ultra Crispy Honey-Kalocsai Chicken & Duck Gyoza with Golden Millet Körítés (Dedicated to Mom * Alexandra / Dra)".to_string(),
            centerpiece: "400°F Dual-Zone Air Fryer Ultra Crisp (Oven Bypassed!) — Juicy Inside, Shatter-Crisp Outside in 14 Real Minutes".to_string(),
            hungarian_korites_layers: vec![
                "Saffron & toasted-butter pearl millet (arany köles körítés) shaped in a sacred Fibonacci spiral (+2 School Leftover Boxes packed!)".to_string(),
                "Charred Farmers Market sweet pepper ribbons & quick-pickled red onions".to_string(),
                "Crisp Air-Fried garlic-parsley baby potatoes brushed with Hungarian plum (szilva) glaze".to_string(),
            ],
            kaiseki_visual_accents: vec![
                "24k edible gold-leaf 8-pointed Star (*) crowning the center in honor of Mom (Alexandra / Dra)".to_string(),
                "Crisp lotus-root geometric triangle (△) chip honoring Dad (Chris)".to_string(),
                "PawShaw Kitty (=^._.^=) & PawShaw Doggy (▽・ω・▽) nori stamp with 38% open porcelain negative space (Ma / 間)".to_string(),
            ],
            vessel_and_lighting: "Warm family platter + 1-Liner Air Fryer basket (7-min real cleanup!) under 2700K bonfire hearth glow".to_string(),
            visual_harmony_score: 99,
        }
    }
}
