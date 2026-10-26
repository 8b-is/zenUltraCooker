//! # USA ZIP Local Market, Fridge/Freezer/Pantry Zero-Waste, Appliance Router & Blended `qFamily` Profiles
//!
//! Simplified & productized household heart for `8b-is/zenUltraCooker` v2.0.0:
//! 1. **USA ZIP Code Local Ads & Farmers Market (`UsaZipMarketFeed`)**:
//!    - Matches weekly grocery store circulars (Aldi, Kroger, Meijer, Costco, Trader Joe's)
//!      and local weekend Farmers Market specials by 5-digit US ZIP code.
//!    - Computes real weekly family savings and instant **DoorDash-Cancel / Late-Notice Rescue**
//!      meals so the family still eats together around the table at least once a day.
//! 2. **Fridge, Freezer & Pantry Zero-Waste Tracker + Household Essentials (`StorageZone`, `InventoryItem`, `HouseholdEssential`)**:
//!    - Tracks what is in the Fridge, Freezer, and Pantry so nothing goes to waste.
//!    - Tracks regular household additions: Toilet Paper, Trash Bags, Laundry Soap, and Dog Poop Bags.
//! 3. **Appliance Status & Air Fryer Ultra Router (`ApplianceKind`, `KitchenAppliance`)**:
//!    - Tracks favorite cooking appliances and whether an appliance is broken (e.g., `Oven == Broken`).
//!    - Honors Dra's law: **"Air Fryer is the ultra appliance of all time"** — automatically
//!      rerouting broken-oven meals to the Air Fryer & Stovetop.
//! 4. **Quantum-Family (`qFamily`) Blended Household Profiles (`FamilyMember`, `FamilyMemberProfile`)**:
//!    - Tracks regular headcount, who likes to cook vs. who helps with dishes/cleanup, cooking skill,
//!      allergies, diets & life stages (e.g., Keto, Gluten-Free, Breastfeeding hydration/kcal boost),
//!      school lunch leftovers, and the **College Son on Speakerphone (`🔊`)** at the stove/table.

use crate::mem16_wave::Standing;

/// Family member identifier around the Blended Quantum-Family (`qFamily`) Hearth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FamilyMember {
    /// Mom (`*` Alex / Alexandra / Dra) — CEO of the Hearth, Family Organizer & Star Center.
    MomAlexandra,
    /// Dad (`△` Chris) — Guardian Pillar, Grill/Broth Master & School Pickup Driver.
    DadChris,
    /// Peter (`◯`) — Cyber-Zen Architect, Air Fryer Crisping & Körítés Plating.
    Peter,
    /// Nate (`◇`) — Sous-Chef Strategist, Bulk Pantry Optimizer & Dish-Zen Partner.
    Nate,
    /// Son in College (`🔊`) — Joins dinner live on Speakerphone around the stove & table!
    CollegeSonSpeakerphone,
}

impl FamilyMember {
    pub const ALL: [Self; 5] = [
        Self::MomAlexandra,
        Self::DadChris,
        Self::Peter,
        Self::Nate,
        Self::CollegeSonSpeakerphone,
    ];

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::MomAlexandra => "Mom (* Alex / Dra)",
            Self::DadChris => "Dad (△ Chris)",
            Self::Peter => "Peter (◯ Zen Tenzo)",
            Self::Nate => "Nate (◇ Sous & Dishes)",
            Self::CollegeSonSpeakerphone => "College Son (🔊 On Speakerphone)",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Self::MomAlexandra => "*",
            Self::DadChris => "△",
            Self::Peter => "◯",
            Self::Nate => "◇",
            Self::CollegeSonSpeakerphone => "🔊",
        }
    }
}

/// How each blended family member participates in the daily meal ritual.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KitchenParticipationRole {
    /// Loves cooking & leading the Air Fryer / Stovetop hearth.
    LeadCook,
    /// Helps chop, marinate, or watch the timer.
    SousPrepHelper,
    /// Prefers not to cook, so rocks the Dish-Zen cleanup & packs school leftovers!
    DishZenAndLeftovers,
    /// Joins remotely on speakerphone while the family cooks & eats together.
    SpeakerphoneTableSeat,
}

impl KitchenParticipationRole {
    pub fn label(&self) -> &'static str {
        match self {
            Self::LeadCook => "Lead Cook (Loves Cooking)",
            Self::SousPrepHelper => "Sous-Prep & Marinate Helper",
            Self::DishZenAndLeftovers => "Dish-Zen Cleanup & School Leftover Packer",
            Self::SpeakerphoneTableSeat => "On Speakerphone at the Stove/Table (Always Welcome)",
        }
    }
}

///Personalized Quantum-Family (`qFamily`) member profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FamilyMemberProfile {
    pub member: FamilyMember,
    pub role: KitchenParticipationRole,
    /// Cooking skill tier (`1..=5`, where 1 is beginner/toast & 5 is fearless).
    pub cooking_skill_tier: u8,
    /// Available weekday time in minutes (including cleanup!).
    pub available_weekday_mins: u16,
    /// Allergies, dietary restrictions, or life-stage nutrition needs (e.g., breastfeeding, keto, nut-free).
    pub dietary_and_life_stage_tags: Vec<String>,
    /// Favorite taste notes to keep meals in rotation.
    pub taste_preferences: String,
    /// Whether this member takes packed leftovers for school/work lunch the next day.
    pub takes_school_or_work_leftovers: bool,
    /// Whether present at the table tonight (or connected via speakerphone).
    pub present_for_dinner_tonight: bool,
}

/// USA ZIP Code local grocery ad circular & weekend Farmers Market feed.
#[derive(Debug, Clone, PartialEq)]
pub struct UsaZipMarketDeal {
    pub store_or_market: String,
    pub item_name: String,
    pub regular_price_usd: f64,
    pub sale_price_usd: f64,
    pub unit_label: String,
    pub is_farmers_market_local: bool,
}

impl UsaZipMarketDeal {
    pub fn savings_usd(&self) -> f64 {
        ((self.regular_price_usd - self.sale_price_usd).max(0.0) * 100.0).round() / 100.0
    }
}

/// Local market context resolved from a 5-digit USA ZIP code.
#[derive(Debug, Clone, PartialEq)]
pub struct UsaZipMarketFeed {
    pub zip_code: String,
    pub city_state: String,
    pub weekly_family_budget_usd: f64,
    pub projected_weekly_spend_usd: f64,
    pub doordash_cancel_saved_usd: f64,
    pub local_deals: Vec<UsaZipMarketDeal>,
}

impl UsaZipMarketFeed {
    /// Verify whether the ZIP code is a valid 5-digit US postal code and budget is within bounds.
    pub fn standing(&self) -> Standing {
        let valid_zip =
            self.zip_code.len() == 5 && self.zip_code.chars().all(|c| c.is_ascii_digit());
        if valid_zip && self.projected_weekly_spend_usd <= self.weekly_family_budget_usd {
            Standing::Verified
        } else if valid_zip {
            Standing::Plausible
        } else {
            Standing::Unobserved
        }
    }

    pub fn total_weekly_ad_savings_usd(&self) -> f64 {
        let sum: f64 = self
            .local_deals
            .iter()
            .map(UsaZipMarketDeal::savings_usd)
            .sum();
        (sum * 100.0).round() / 100.0
    }
}

/// Storage zone inside the home so nothing in Fridge, Freezer, or Pantry goes to waste.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageZone {
    /// Fresh Refrigerator shelf & humidity crisper.
    Fridge,
    /// Deep Freezer (frozen proteins, berries, batch stock, backup Air Fryer rescue meals).
    Freezer,
    /// Dry Pantry & Costco bulk shelf (grains, spices, canned goods, oils).
    Pantry,
}

impl StorageZone {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Fridge => "Fridge ❄️",
            Self::Freezer => "Freezer 🧊",
            Self::Pantry => "Pantry 🥫",
        }
    }
}

/// Source of an ingredient in the simplified USA family kitchen ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupplyOrigin {
    /// Tracked in Fridge, Freezer, or Pantry.
    HomeFridgeFreezerPantry,
    /// Local Weekly Grocery Store Ad (Aldi, Kroger, Meijer, Costco).
    LocalWeeklyAdCircular,
    /// Weekend Local Farmers Market stand.
    LocalFarmersMarket,
}

/// A tracked food item across Fridge, Freezer, and Pantry with zero-waste expiry priority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryItem {
    pub name: String,
    pub zone: StorageZone,
    pub origin: SupplyOrigin,
    pub quantity_label: String,
    pub days_until_expiry: u16,
    pub use_first_alert: bool,
}

/// Non-food household essentials tracked on the same weekly family run (TP, Trash Bags, Laundry Soap, Dog Poop Bags).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HouseholdEssential {
    pub item_name: String,
    pub category: String,
    pub low_stock_alert: bool,
    pub assigned_runner: FamilyMember,
}

/// Kitchen cooking appliance category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplianceKind {
    /// Dra's Law: "Air Fryer is the ultra appliance of all time."
    AirFryerUltra,
    /// Standard kitchen oven (may be broken/offline!).
    ConventionalOven,
    /// Stovetop / Range burners.
    StovetopSkillet,
    /// Instant Pot / Slow Cooker.
    InstantPotSlowCooker,
    /// Outdoor Grill / Campfire Bonfire.
    OutdoorBonfireGrill,
}

/// Tracked status of a family kitchen appliance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KitchenAppliance {
    pub kind: ApplianceKind,
    pub display_name: String,
    pub is_working: bool,
    pub is_ultra_favorite: bool,
    pub status_note: String,
}

impl KitchenAppliance {
    pub fn routing_standing(&self) -> Standing {
        if self.is_working && self.is_ultra_favorite {
            Standing::Verified
        } else if self.is_working {
            Standing::Plausible
        } else {
            Standing::Unobserved
        }
    }
}

/// Blended family schedule, DoorDash cancel recovery & Uninterrupted Dinner Table Sanctuary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlendedFamilyScheduleContext {
    pub usa_zip_code: String,
    pub regular_headcount: u8,
    pub tonight_headcount: u8,
    pub speakerphone_college_son_connected: bool,
    pub late_cancel_or_doordash_rescued: bool,
    pub dinner_table_sanctuary_mins: u16,
    pub pawshaw_trademark_badge: String,
}

impl BlendedFamilyScheduleContext {
    /// Eating together at least once a day (>= 30 mins around the table/stove) is Verified (`+1`).
    pub fn unity_standing(&self) -> Standing {
        if self.dinner_table_sanctuary_mins >= 30 && self.tonight_headcount >= 2 {
            Standing::Verified
        } else if self.tonight_headcount >= 1 {
            Standing::Plausible
        } else {
            Standing::Unobserved
        }
    }
}
