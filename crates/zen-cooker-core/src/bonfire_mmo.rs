//! # Animal Crossing (`<3<3<3`) Pixel-Art Cozy Bonfire MMO & `PawShaw Kitty & Doggy` Trademark
//!
//! - **Center of Family ~=== Bonfire (`Animal Crossing <3<3<3` Cozy Pixel-Art MMO)**:
//!   When Mom cooks, the family comes together — and eating together at least once a day
//!   drastically reduces the risk of drug abuse and out-of-bounds addictions.
//! - **PawShaw Kitty (`=^._.^= ∫`) & PawShaw Doggy (`▽・ω・▽`) Trademark**:
//!   Our beloved twin pixel companions guarding the kitchen hearth, reminding the crew to grab
//!   dog poop bags, cheering on Dish-Zen cleanup, and keeping the bonfire warm.
//! - **Everyone Is Welcome Around the Stove or Table**:
//!   Includes **Mom (`*` Alexandra / Dra)**, **Dad (`△` Chris)**, **Peter (`◯`)**, **Nate (`◇`)**,
//!   **College Son on Speakerphone (`🔊`)**, **PawShaw Kitty**, and **PawShaw Doggy**.

use crate::telemetry::FamilyMember;

/// The Three Minds (*San-shin* / 三心) of Zen Cooking (*Tenzo Kyōkun*) in the Pixel-Art Bonfire Village.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZenMindState {
    /// 喜心 *Kishin* — Joyful, grateful energy in cooking, Dish-Zen cleanup, or setting the table.
    KishinJoyful,
    /// 老心 *Rōshin* — Deep maternal/paternal tenderness watching over the blended family's well-being.
    RoshinNurturing,
    /// 大心 *Daishin* — Vast, welcoming equanimity where everyone has a seat at the bonfire.
    DaishinMagnanimous,
}

impl ZenMindState {
    pub fn description(&self) -> &'static str {
        match self {
            Self::KishinJoyful => {
                "Kishin (喜心 Joyful Mind) — cooking, dishes & school-lunch packing as cozy play"
            }
            Self::RoshinNurturing => {
                "Roshin (老心 Nurturing Mind) — unconditional family love like Mom (*) & Dad (△)"
            }
            Self::DaishinMagnanimous => {
                "Daishin (大心 Magnanimous Mind) — everyone is welcome around the stove or on speakerphone"
            }
        }
    }
}

/// Animal Crossing-style Pixel-Art MMO Avatar around the Family Bonfire Hearth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BonfireAvatar {
    pub handle: String,
    pub class_title: String,
    pub emblem: String,
    pub level: u16,
    pub xp: u32,
    pub pixel_sprite_tag: String,
    pub signature_buff: String,
}

/// A gamified family contribution quest around the kitchen hearth (cooking, dishes, school pickup, speakerphone).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HearthQuestContribution {
    pub contributor: FamilyMember,
    pub quest_name: String,
    pub zen_mind: ZenMindState,
    pub xp_awarded: u32,
    pub hearth_flame_boost: u8,
}

/// An immutable entry in the Family Bonfire Culinary, Photo Moment & Gratitude Journal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BonfireJournalEntry {
    pub timestamp_iso: String,
    pub author: String,
    pub zen_mind: ZenMindState,
    pub reflection: String,
    pub dedication_tag: String,
}

/// Complete state of the Animal Crossing-style Pixel-Art Family Bonfire MMO & Journal session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FamilyBonfireSession {
    pub bonfire_level: u16,
    pub total_party_xp: u32,
    pub flame_warmth_pct: u8,
    pub pawshaw_trademark: String,
    pub avatars: Vec<BonfireAvatar>,
    pub contributions: Vec<HearthQuestContribution>,
    pub journal: Vec<BonfireJournalEntry>,
}

impl FamilyBonfireSession {
    /// Initialize the canonical Blended `qFamily` + `PawShaw Kitty & Doggy` Animal Crossing Pixel-Art Bonfire party.
    pub fn initialize_family_hearth() -> Self {
        let avatars = vec![
            BonfireAvatar {
                handle: "Alex ~ Alexandra / Dra (*)".to_string(),
                class_title: "Grand Star Hearth Mom & Air Fryer Ultra Queen".to_string(),
                emblem: "*".to_string(),
                level: 99,
                xp: 99_999,
                pixel_sprite_tag: "SPRITE_MOM_STAR_CROWN".to_string(),
                signature_buff: "When Mom Cooks, Family Comes Together: +100% Unity, Zero Waste & 528 Hz Love".to_string(),
            },
            BonfireAvatar {
                handle: "Chris (△)".to_string(),
                class_title: "Guardian Pillar, School Pickup & Farmers Market Scout".to_string(),
                emblem: "△".to_string(),
                level: 95,
                xp: 88_400,
                pixel_sprite_tag: "SPRITE_DAD_PILLAR_CAPE".to_string(),
                signature_buff: "Pillar Foundation (△): Never Misses School Pickup, Local Ad Deals & 288 Hz Bass".to_string(),
            },
            BonfireAvatar {
                handle: "Peter (◯)".to_string(),
                class_title: "Cyber-Zen Tenzo & Pixel-Art Village Architect".to_string(),
                emblem: "◯".to_string(),
                level: 88,
                xp: 64_200,
                pixel_sprite_tag: "SPRITE_PETER_ZEN_BANDANA".to_string(),
                signature_buff: "Anti-Rachael Ray Honest Timer: Real Prep + 14m Air Fryer Crisp + Körítés Magic".to_string(),
            },
            BonfireAvatar {
                handle: "Nate (◇)".to_string(),
                class_title: "Dish-Zen Cleanup Champion & School Leftover Strategist".to_string(),
                emblem: "◇".to_string(),
                level: 88,
                xp: 64_200,
                pixel_sprite_tag: "SPRITE_NATE_GALACTIC_APRON".to_string(),
                signature_buff: "7-Minute Dish-Zen Blitz: Packs Tomorrow's School Lunches & Zeroes Sink Clutter".to_string(),
            },
            BonfireAvatar {
                handle: "College Son (🔊)".to_string(),
                class_title: "Speakerphone Stove Companion & Campus Taster".to_string(),
                emblem: "🔊".to_string(),
                level: 85,
                xp: 52_000,
                pixel_sprite_tag: "SPRITE_SON_SPEAKERPHONE_WAVE".to_string(),
                signature_buff: "Speakerphone Seat at the Table: Distance Dissolves When We Cook Dinner Together".to_string(),
            },
            BonfireAvatar {
                handle: "PawShaw Kitty & Doggy (^^)".to_string(),
                class_title: "Official PawShaw Trademark Duo (Pixel Neko & Inugami)".to_string(),
                emblem: "=^._.^= ▽・ω・▽".to_string(),
                level: 77,
                xp: 42_000,
                pixel_sprite_tag: "SPRITE_PAWSHAW_KITTY_DOGGY_DUO".to_string(),
                signature_buff: "PawShaw Trademark Blessing: Reminds You to Buy Dog Poop Bags & Donuts for Advisor Lunch!".to_string(),
            },
        ];

        let contributions = vec![
            HearthQuestContribution {
                contributor: FamilyMember::MomAlexandra,
                quest_name: "4:15 PM Marinate & 14-Min Air Fryer Ultra Feast (Oven Broken -> Bypassed!)"
                    .to_string(),
                zen_mind: ZenMindState::RoshinNurturing,
                xp_awarded: 1_500,
                hearth_flame_boost: 35,
            },
            HearthQuestContribution {
                contributor: FamilyMember::DadChris,
                quest_name: "3:30 PM School Pickup + Local ZIP Ad Run + Dog Poop Bags & Trash Bags".to_string(),
                zen_mind: ZenMindState::DaishinMagnanimous,
                xp_awarded: 1_200,
                hearth_flame_boost: 25,
            },
            HearthQuestContribution {
                contributor: FamilyMember::Peter,
                quest_name:
                    "Real-Prep Chopping (10m Honest Prep), Golden Millet Körítés & Photo Moment Capture"
                        .to_string(),
                zen_mind: ZenMindState::KishinJoyful,
                xp_awarded: 1_200,
                hearth_flame_boost: 20,
            },
            HearthQuestContribution {
                contributor: FamilyMember::Nate,
                quest_name: "7-Min Dish-Zen Cleanup + Packing 2 School Lunch Leftover Boxes"
                    .to_string(),
                zen_mind: ZenMindState::KishinJoyful,
                xp_awarded: 1_200,
                hearth_flame_boost: 10,
            },
            HearthQuestContribution {
                contributor: FamilyMember::CollegeSonSpeakerphone,
                quest_name: "Calling In Live on Speakerphone While Dinner Cooks Around the Stove"
                    .to_string(),
                zen_mind: ZenMindState::DaishinMagnanimous,
                xp_awarded: 1_000,
                hearth_flame_boost: 10,
            },
        ];

        let total_party_xp: u32 = avatars.iter().map(|a| a.xp).sum::<u32>()
            + contributions.iter().map(|c| c.xp_awarded).sum::<u32>();

        let flame_warmth_pct = contributions
            .iter()
            .map(|c| c.hearth_flame_boost)
            .sum::<u8>()
            .min(100);

        let journal = vec![
            BonfireJournalEntry {
                timestamp_iso: "2026-10-26T18:30:00-04:00".to_string(),
                author: "Mom (* Alex / Dra), Dad (△ Chris), Peter (◯), Nate (◇) & College Son (🔊)".to_string(),
                zen_mind: ZenMindState::RoshinNurturing,
                reflection: "When a family eats together at least once a day, it protects everyone we love. The oven broke, DoorDash canceled, food prices rose — so we fired up the Air Fryer Ultra, used our Fridges/Freezer/Pantry + local ZIP ads, put our son in college on speakerphone, and ate together in 24 honest minutes!".to_string(),
                dedication_tag: "Mom===Alex~Dra<3".to_string(),
            },
            BonfireJournalEntry {
                timestamp_iso: "2026-10-26T18:45:00-04:00".to_string(),
                author: "PawShaw Kitty (=^._.^=) & PawShaw Doggy (▽・ω・▽)".to_string(),
                zen_mind: ZenMindState::KishinJoyful,
                reflection: "Woof & Nyaa~! 5/5 Stars rated! Photo moment saved to rotation, 2 school leftover boxes packed in the fridge, dog poop bags & laundry soap restocked, and Advisor Lunch donuts reminder set for tomorrow morning!".to_string(),
                dedication_tag: "PawShaw-Kitty-Doggy-TM".to_string(),
            },
        ];

        Self {
            bonfire_level: 100,
            total_party_xp,
            flame_warmth_pct,
            pawshaw_trademark: "PawShaw Kitty (=^._.^= ∫) & PawShaw Doggy (▽・ω・▽) — Everyone Is Welcome at the Table".to_string(),
            avatars,
            contributions,
            journal,
        }
    }
}
