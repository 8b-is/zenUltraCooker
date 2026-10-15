//! # `zen-cooker-quant-tern` — dedicated ternary inference & wave crate
//!
//! The `QUANT-TERN` expansion of [`zenUltraCooker`](https://github.com/8b-is/zenUltraCooker).
//! One ternary law, two alphabets, never conflated:
//!
//! - **Gate / Affect Alphabet** (`crush-love-dev`): `-1 -> 0b00 (refuse)`,
//!   `0 -> 0b01 (rest)`, `+1 -> 0b10 (affirm)` — see
//!   [`zen_cooker_core::quant_tern::Trit::gate_code`].
//! - **Weight Alphabet** (`8b-is-engine/crates/ternary`): `0 -> 0b00`,
//!   `+1 -> 0b01`, `-1 -> 0b10` — [`zen_cooker_core::quant_tern::Trit::weight_code`].
//!
//! Modules:
//! - [`pack5_wupz`] — Kintsugi base-3 packing: 5 trits per `u8` (3^5 = 243 ≤ 255)
//!   for zero-dep wire transport, round-trip proven by property tests.
//! - [`spectral_guard`] — power-iteration `s_max(W)` estimator checked against the
//!   KOMPRESS v2 Marchenko-Pastur rigidity bound before any `BIND` stage.
//! - [`ternary_recipe_router`] — multi-teacher consensus distillation router
//!   (geometric-mean agreement across Mom `*`, Dad `△`, Peter `◯`, Nate `◇`)
//!   emitting Scoville SHU, *körítés* ratios and 432×528×288 Hz wave envelopes
//!   with zero floating-point multiplies in the inner loop.
//! - [`hearth_reference`] — the pure `i32` reference for the Metal ternary
//!   hearth kernel, pinned by golden constants.
//!
//! Dedicated to Mom (`*` Alex ~ Alexandra) — co-creator of the Family Wave
//! Theory (*A Hullámelmélet*). So long, and thanks for all the fish. <3

pub mod hearth_reference;
pub mod pack5_wupz;
pub mod spectral_guard;
pub mod ternary_recipe_router;

pub use hearth_reference::ternary_hearth_i32;
pub use pack5_wupz::{pack5, unpack5};
pub use spectral_guard::{guard_bind, mp_bound, spectral_smax, GuardVerdict};
pub use ternary_recipe_router::{route_recipe, ZenRecipe};
