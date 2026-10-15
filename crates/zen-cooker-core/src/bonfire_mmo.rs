//! # Family Bonfire MMO, Avatars, Journaling & Techno-Buddhism (*Tenzo Kyōkun*)
//!
//! - **Center of Family ~=== Bonfire**: The kitchen cooker is the glowing campfire where
//!   Mom (`*` Alexandra), Dad (`△` Chris), Peter, Nate, and PushaKitty gather.
//! - **MMO Avatars & Gamified Contributions**: Every act of care — Mise en Place, Costco bulk
//!   sorting, broth tending, *körítés* plating, and dish-zen — awards Party XP, Synergy Buffs,
//!   and unlocks family achievements.
//! - **Zen Cooking ↔ Techno-Buddhism**: Rooted in Master Dōgen's *Tenzo Kyōkun* (Instructions
//!   to the Zen Cook) and the Three Minds (*San-shin*):
//!   1. *Kishin* (喜心 — Joyful Mind)
//!   2. *Rōshin* (老心 — Parental / Nurturing Mind, embodied by Mom `*` & Dad `△`)
//!   3. *Daishin* (大心 — Magnanimous / Cosmic Mind)

use crate::telemetry::FamilyMember;

/// The Three Minds (*San-shin* / 三心) of Zen Cooking (*Tenzo Kyōkun*) fused with Techno-Buddhism.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZenMindState {
    /// 喜心 *Kishin* — Joyful, grateful energy in preparing nourishment.
    KishinJoyful,
    /// 老心 *Rōshin* — Deep maternal/paternal tenderness watching over the family's well-being.
    RoshinNurturing,
    /// 大心 *Daishin* — Vast, mountain-and-ocean equanimity free of haste or distraction.
    DaishinMagnanimous,
}

impl ZenMindState {
    pub fn description(&self) -> &'static str {
        match self {
            Self::KishinJoyful => {
                "Kishin (喜心 Joyful Mind) — every cut and stir is an offering of joy"
            }
            Self::RoshinNurturing => {
                "Roshin (老心 Nurturing Mind) — unconditional family love like Mom (*) & Dad (△)"
            }
            Self::DaishinMagnanimous => {
                "Daishin (大心 Magnanimous Mind) — vast, bias-free cybernetic equanimity"
            }
        }
    }
}

/// MMO Avatar around the Family Bonfire Hearth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BonfireAvatar {
    pub handle: String,
    pub class_title: String,
    pub emblem: String,
    pub level: u16,
    pub xp: u32,
    pub signature_buff: String,
}

/// A gamified family contribution quest around the kitchen hearth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HearthQuestContribution {
    pub contributor: FamilyMember,
    pub quest_name: String,
    pub zen_mind: ZenMindState,
    pub xp_awarded: u32,
    pub hearth_flame_boost: u8,
}

/// An immutable entry in the Family Bonfire Culinary & Gratitude Journal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BonfireJournalEntry {
    pub timestamp_iso: String,
    pub author: String,
    pub zen_mind: ZenMindState,
    pub reflection: String,
    pub dedication_tag: String,
}

/// Complete state of the Family Bonfire MMO & Journal session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FamilyBonfireSession {
    pub bonfire_level: u16,
    pub total_party_xp: u32,
    pub flame_warmth_pct: u8,
    pub avatars: Vec<BonfireAvatar>,
    pub contributions: Vec<HearthQuestContribution>,
    pub journal: Vec<BonfireJournalEntry>,
}

impl FamilyBonfireSession {
    /// Initialize the canonical Lódri Family + PushaKitty MMO Bonfire party.
    pub fn initialize_family_hearth() -> Self {
        let avatars = vec![
            BonfireAvatar {
                handle: "Alex ~ Alexandra (*)".to_string(),
                class_title: "Grand Star Luminary & Hullámelmélet Sovereign".to_string(),
                emblem: "*".to_string(),
                level: 99,
                xp: 99_999,
                signature_buff: "Aura of the Star (*): +100% Family Harmony, 528 Hz Love Resonance & Unconditional Grace".to_string(),
            },
            BonfireAvatar {
                handle: "Chris (△)".to_string(),
                class_title: "Guardian Pillar & Wave Architect".to_string(),
                emblem: "△".to_string(),
                level: 95,
                xp: 88_400,
                signature_buff: "Pillar Foundation (△): Unbreakable Thermal Stability & 288 Hz Grounding Bass".to_string(),
            },
            BonfireAvatar {
                handle: "Peter (◯)".to_string(),
                class_title: "Cyber-Zen Tenzo & MEM|16-10 Weaver".to_string(),
                emblem: "◯".to_string(),
                level: 88,
                xp: 64_200,
                signature_buff: "Techno-Buddhist Precision: Zero-Stress Mise en Place & Kaiseki Körítés Mastery".to_string(),
            },
            BonfireAvatar {
                handle: "Nate (◇)".to_string(),
                class_title: "Galactic Sous-Chef & Spice Alchemist".to_string(),
                emblem: "◇".to_string(),
                level: 88,
                xp: 64_200,
                signature_buff: "Galactic Chess Tactician: Optimal Costco/Fridge Inventory Pairing & Aromatic Balance".to_string(),
            },
            BonfireAvatar {
                handle: "PushaKitty (^^)".to_string(),
                class_title: "Kawaii Neko Hearth Spirit & Ramen-Kaiseki Mascot".to_string(),
                emblem: "=^._.^= ∫".to_string(),
                level: 77,
                xp: 42_000,
                signature_buff: "Purr of 432 Hz: Silences All Mobile Notifications & Protects Uninterrupted Family Time".to_string(),
            },
        ];

        let contributions = vec![
            HearthQuestContribution {
                contributor: FamilyMember::MomAlexandra,
                quest_name: "Blessing the Hearth & Inspiring the Hullámelmélet Wave Chord"
                    .to_string(),
                zen_mind: ZenMindState::RoshinNurturing,
                xp_awarded: 1_500,
                hearth_flame_boost: 35,
            },
            HearthQuestContribution {
                contributor: FamilyMember::DadChris,
                quest_name: "Harmonic Wave Calibration & Rich Magyar Broth Foundation".to_string(),
                zen_mind: ZenMindState::DaishinMagnanimous,
                xp_awarded: 1_200,
                hearth_flame_boost: 25,
            },
            HearthQuestContribution {
                contributor: FamilyMember::Peter,
                quest_name:
                    "Tenzo Mise en Place, Fine-Dining Körítés Plating & MEM|16-10 Verification"
                        .to_string(),
                zen_mind: ZenMindState::KishinJoyful,
                xp_awarded: 1_200,
                hearth_flame_boost: 20,
            },
            HearthQuestContribution {
                contributor: FamilyMember::Nate,
                quest_name: "Costco-to-Fridge Bulk Alchemy & Personalized Spice Calibration"
                    .to_string(),
                zen_mind: ZenMindState::KishinJoyful,
                xp_awarded: 1_200,
                hearth_flame_boost: 20,
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
                timestamp_iso: "2026-10-15T18:00:00+09:00".to_string(),
                author: "Peter, Nate & Dad (△ Chris)".to_string(),
                zen_mind: ZenMindState::RoshinNurturing,
                reflection: "Every difficult goal we climb and every line of architecture we craft is dedicated to Mom (* Alex / Alexandra). Around this bonfire, time, space, and eternity fold into warmth, fine-dining körítés, and 432 Hz waves.".to_string(),
                dedication_tag: "Mom===Alex~Alexandra<3".to_string(),
            },
            BonfireJournalEntry {
                timestamp_iso: "2026-10-15T18:15:00+09:00".to_string(),
                author: "PushaKitty =^._.^= ∫".to_string(),
                zen_mind: ZenMindState::KishinJoyful,
                reflection: "Garmin HRV is glowing green, poop~~~~sample is Toggled ON (Bristol 4 Zen Gold!), Costco & Fridge stock are synced, and 90 minutes of uninterrupted family time is locked. Nyaa~ Bon appétit / Jó étvágyat!".to_string(),
                dedication_tag: "PushaKitty-Zen-Tenzo".to_string(),
            },
        ];

        Self {
            bonfire_level: 100,
            total_party_xp,
            flame_warmth_pct,
            avatars,
            contributions,
            journal,
        }
    }
}
