//! # `zen-cooker` — PushaKitty Cyber-Zen Hearth CLI (`8b-is/zenUltraCooker`)
//!
//! Runs a full E2E simulation of `zenUltraCooker`, printing the Japan-style cute
//! PushaKitty mascot, the `MEM|16-10` Wave Theory chord for Mom (`*` Alexandra),
//! Garmin + `poop~~~~sample` telemetry, Costco/Fridge inventory, Fine-Dining *Körítés*
//! plating, and Family Bonfire MMO status.

use std::env;
use zen_cooker_core::execute_family_hearth_cycle;

const PUSHAKITTY_BANNER: &str = r#"
  ╭──────────────────────────────────────────────────────────────────────────────╮
  │    ╱|、       8b-is / zenUltraCooker v1.0.0 — PushaKitty Tenzo Hearth       │
  │   (˚ˎ 。7  *  Dedicated with All Our Family Love to Mom (* Alex ~ Alexandra) │
  │    |、˜〵     From Peter (◯), Nate (◇) & Dad (△ Chris) — Bonfire == Family   │
  │    じしˍ,)ノ  Music ~= Waves ~ MEM|16-10 (POP->REFUSE->BIND->TRANSFORM->...) │
  ╰──────────────────────────────────────────────────────────────────────────────╯
"#;

fn main() {
    let args: Vec<String> = env::args().collect();
    let poop_toggled_on = !args.iter().any(|a| a == "--poop-sample-off");

    let plan = execute_family_hearth_cycle(poop_toggled_on);

    println!("{PUSHAKITTY_BANNER}");
    println!("  ✦ DEDICATION : {}", plan.dedication_banner);
    println!(
        "  ✦ AREA & TIME: {} | Family Arrival ETA: {}m | Uninterrupted Sanctuary: {}m [{}]",
        plan.area_context.area_name,
        plan.area_context.family_convergence_eta_mins,
        plan.area_context.uninterrupted_foundation_mins,
        plan.area_context.foundation_standing()
    );
    println!(
        "  ✦ GUT TOGGLE : poop~~~~sample is Toggled {} ----> [Standing: {} | Bristol: {}/7 | Diversity: {}%]",
        if plan.poop_sample_telemetry.toggled_on {
            "ON "
        } else {
            "OFF"
        },
        plan.poop_sample_telemetry.standing(),
        plan.poop_sample_telemetry.bristol_scale,
        plan.poop_sample_telemetry.microbiome_diversity
    );
    println!();

    println!("  ┌── 1. MUSIC ~= WAVES ~ MEM|16-10 HULLÁMELMÉLET FOR MOM (* ALEXANDRA) ─────────┐");
    println!("  │ Chord     : {}", plan.wavechord.dedication_chord_name);
    println!(
        "  │ Harmonics : {:.2} Hz Carrier | {:.1} Hz Mom (*) Star | {:.1} Hz Dad (△) Pillar | {:.2} Hz Binaural",
        plan.wavechord.carrier_hz,
        plan.wavechord.overtone_mom_star_hz,
        plan.wavechord.bass_dad_pillar_hz,
        plan.wavechord.binaural_calm_hz
    );
    println!(
        "  │ Waveform  : [{}] (Coherence: {:.1}%)",
        plan.wavechord.ascii_wave,
        plan.wavechord.coherence * 100.0
    );
    println!(
        "  │ Escalation: mem16 (16B) -> 32 (32B XOR:0x{:02x}) ~~> 8k (bleeding-edge 8192B/8192-trit: 0x{:016x})",
        plan.wavechord.mem32_wave_frame[31],
        plan.wavechord.bleeding_edge_8k_digest
    );
    println!("  └──────────────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("  ┌── 2. GARMIN BIOMETRICS × PERSONALIZED SPICE & PREBIOTIC ENGINE ──────────────┐");
    for profile in &plan.spice_profiles {
        println!(
            "  │ [{}] {:<24} -> {:>4} SHU | Sweet Paprika: {:.1}g | Hot: {:.2}g | Yuzu-Sansho: {:.2}g | Prebiotic: {}g",
            profile.member.symbol(),
            profile.member.display_name(),
            profile.target_scoville_shu,
            profile.sweet_paprika_grams,
            profile.hot_paprika_grams,
            profile.yuzu_sansho_grams,
            profile.prebiotic_boost_grams
        );
    }
    println!("  └──────────────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("  ┌── 3. FINE DINING & HUNGARIAN KÖRÍTÉS (THE LOOKS — MICHELIN × KAISEKI) ───────┐");
    println!("  │ Dish   : {}", plan.korites_plating.dish_title);
    println!("  │ Thermal: {}", plan.korites_plating.centerpiece);
    for (i, layer) in plan
        .korites_plating
        .hungarian_korites_layers
        .iter()
        .enumerate()
    {
        println!("  │ Körítés #{}: {}", i + 1, layer);
    }
    for accent in &plan.korites_plating.kaiseki_visual_accents {
        println!("  │ Kaiseki  : {}", accent);
    }
    println!(
        "  │ Vessel   : {} (Harmony: {}/100)",
        plan.korites_plating.vessel_and_lighting, plan.korites_plating.visual_harmony_score
    );
    println!("  └──────────────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("  ┌── 4. CENTER OF FAMILY ~=== BONFIRE MMO & TECHNO-BUDDHIST JOURNAL ────────────┐");
    println!(
        "  │ Bonfire Level {} | Party XP: {} | Flame Warmth: {}%",
        plan.bonfire_session.bonfire_level,
        plan.bonfire_session.total_party_xp,
        plan.bonfire_session.flame_warmth_pct
    );
    for avatar in &plan.bonfire_session.avatars {
        println!(
            "  │   {:<10} {:<22} Lv.{:<3} — {}",
            avatar.emblem, avatar.handle, avatar.level, avatar.class_title
        );
    }
    println!("  ├──────────────────────────────────────────────────────────────────────────────┤");
    for entry in &plan.bonfire_session.journal {
        println!(
            "  │ [{}] {} ({}):",
            entry.dedication_tag,
            entry.author,
            entry.zen_mind.description()
        );
        println!("  │   \"{}\"", entry.reflection);
    }
    println!("  └──────────────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("  ┌── 5. MEM|16-10 GOVERNING SEQUENCE RECEIPTS (POP -> COLLAPSE) ────────────────┐");
    for receipt in &plan.mem16_receipts {
        println!(
            "  │ {:<9} [{:<10}] 0x{:016x} | {}",
            format!("{}", receipt.stage),
            format!("{}", receipt.standing),
            receipt.state_hash,
            receipt.summary
        );
    }
    println!(
        "  │ FINAL COLLAPSE LEDGER HASH: 0x{:016x}",
        plan.final_collapse_hash
    );
    println!("  └──────────────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("  ┌── 6. QUANT-TERN BRIDGE (crush-love-dev × pureQTern × quanttern · 全) ────────┐");
    println!(
        "  │ Tribute   : {}",
        plan.quant_tern_receipt.fish_tribute_for_mom
    );
    println!(
        "  │ Affect Tag: {} | Gate Trits: {:?} | Scale γ = {:.4}",
        plan.quant_tern_receipt.qt_hex_tag,
        plan.quant_tern_receipt.gate_trits,
        plan.quant_tern_receipt.gamma_scale
    );
    println!(
        "  │ Matvec i32: {:?} | Golden Hash: 0x{:016x}",
        plan.quant_tern_receipt.matvec_logits, plan.quant_tern_receipt.golden_hash
    );
    println!("  └──────────────────────────────────────────────────────────────────────────────┘");
}
