//! # `zen-cooker` v2.0.0 — PawShaw Kitty & Doggy Quantum-Family Household Organizer CLI (`8b-is/zenUltraCooker`)
//!
//! Runs a full E2E simulation of `zenUltraCooker` v2.0.0:
//! - USA ZIP Code Local Grocery Ads & Farmers Market Savings
//! - Fridge, Freezer & Pantry Zero-Waste Tracker + Household Essentials (Dog Poop Bags, TP, Trash Bags, Laundry Soap, Donuts)
//! - Appliance Status Tracker (Oven Broken -> Auto-Rerouted to Air Fryer Ultra ★)
//! - Anti-Rachael Ray Honest Prep + Cook + Cleanup Recipe Vault (`zenFamily` Share vs. Family Secret + Star Ratings & Photo Moments)
//! - Personalized Profile-Based `quantAI` Family Assistant Nudges
//! - Animal Crossing (`<3<3<3`) Pixel-Art Cozy Bonfire MMO & `PawShaw Kitty (=^._.^=) & Doggy (▽・ω・▽)` Trademark
//! - Stripe + Apple Pay + Google Pay & 4-Burner Sovereign Hearth Firewall (`Gondolkodom, Theat vagyOK -> just BE <3`)

use std::env;
use zen_cooker_core::execute_family_hearth_cycle;

const PAWSHAW_BANNER: &str = r#"
  ╭──────────────────────────────────────────────────────────────────────────────────╮
  │   ╱|、  ▽・ω・▽   8b-is / zenUltraCooker v2.0.0 — PawShaw Kitty & Doggy Hearth  │
  │  (˚ˎ 。7   *      When Mom (* Alex / Dra) Cooks, the Family Comes Together <3    │
  │   |、˜〵   🔊     USA ZIP Ads • Air Fryer Ultra ★ • Fridge/Freezer/Pantry • MMO  │
  │   じしˍ,)ノ       Gondolkodom, Theat vagyOK -> cogito ergo summa summarum | BE   │
  ╰──────────────────────────────────────────────────────────────────────────────────╯
"#;

fn main() {
    let args: Vec<String> = env::args().collect();
    let air_fryer_verified = !args.iter().any(|a| a == "--unverified-appliance");

    let plan = execute_family_hearth_cycle(air_fryer_verified);

    println!("{PAWSHAW_BANNER}");
    println!("  ✦ DEDICATION : {}", plan.dedication_banner);
    println!(
        "  ✦ TRADEMARK  : {}",
        plan.schedule_context.pawshaw_trademark_badge
    );
    println!(
        "  ✦ USA ZIP    : {} ({}) | Tonight Headcount: {}/{} (College Son on Speakerphone 🔊: {})",
        plan.schedule_context.usa_zip_code,
        plan.zip_market_feed.city_state,
        plan.schedule_context.tonight_headcount,
        plan.schedule_context.regular_headcount,
        if plan.schedule_context.speakerphone_college_son_connected {
            "LIVE AT TABLE"
        } else {
            "OFF"
        }
    );
    println!(
        "  ✦ BUDGET     : Weekly Spend ${:.2} / ${:.2} Budget | Ad Savings: ${:.2} | DoorDash Cancel Saved: ${:.2} [{}]",
        plan.zip_market_feed.projected_weekly_spend_usd,
        plan.zip_market_feed.weekly_family_budget_usd,
        plan.zip_market_feed.total_weekly_ad_savings_usd(),
        plan.zip_market_feed.doordash_cancel_saved_usd,
        plan.zip_market_feed.standing()
    );
    println!();

    println!("  ┌── 1. APPLIANCE ROUTER (AIR FRYER ULTRA ★ vs. BROKEN OVEN BYPASS) ─────────────┐");
    for app in &plan.appliances {
        println!(
            "  │ [{}] {:<48} -> {}",
            if app.is_working { "ONLINE " } else { "BROKEN " },
            app.display_name,
            app.status_note
        );
    }
    println!("  └───────────────────────────────────────────────────────────────────────────────┘");
    println!();

    println!(
        "  ┌── 2. FRIDGE ❄️ / FREEZER 🧊 / PANTRY 🥫 ZERO-WASTE + HOUSEHOLD ESSENTIALS 🧻 ──┐"
    );
    for item in &plan.inventory_used {
        println!(
            "  │ {:<10} {:<52} | Exp: {:>3}d | {}",
            item.zone.label(),
            item.name,
            item.days_until_expiry,
            item.quantity_label
        );
    }
    println!("  ├───────────────────────────────────────────────────────────────────────────────┤");
    for ess in &plan.household_essentials {
        println!(
            "  │ [{}] {:<50} | Runner: {}",
            if ess.low_stock_alert {
                "ADD NOW"
            } else {
                "STOCKED"
            },
            ess.item_name,
            ess.assigned_runner.display_name()
        );
    }
    println!("  └───────────────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("  ┌── 3. PER-FAMILY + PER-PROFILE quantAI ASSISTANT NUDGES ───────────────────────┐");
    for nudge in &plan.quant_ai_nudges {
        println!(
            "  │ [{}] {:<22} ({}) -> {}",
            nudge.target_member.symbol(),
            nudge.target_member.display_name(),
            nudge.trigger_time_label,
            nudge.message
        );
    }
    println!("  └───────────────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("  ┌── 4. HONEST REAL-PREP & CLEANUP RECIPES (ANTI-RACHAEL RAY LAW + ROTATION) ────┐");
    for r in &plan.recipes_in_rotation {
        println!(
            "  │ ★ {:.1}/5.0 | {} [{}]",
            r.family_rating_stars,
            r.title,
            r.visibility.badge()
        );
        println!(
            "  │   Honest Total: {}m (Unwashed Prep: {}m + Air Fryer Cook: {}m + Real Dish Cleanup: {}m | Marinate Lead: {}m)",
            r.honest_total_kitchen_mins(),
            r.unwashed_prep_mins,
            r.active_cook_mins,
            r.cleanup_and_dishes_mins,
            r.marinate_or_thaw_lead_mins
        );
        println!(
            "  │   Cost/Serving: ${:.2} | School Lunch Leftover Boxes: +{} | Moments: {}",
            r.cost_per_serving_usd,
            r.school_leftover_boxes,
            r.photo_moments.join(" • ")
        );
    }
    println!("  └───────────────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("  ┌── 5. ANIMAL CROSSING (<3<3<3) PIXEL-ART BONFIRE MMO & PAWSHAW TRADEMARK ──────┐");
    println!(
        "  │ Bonfire Level {} | Party XP: {} | Flame Warmth: {}% | {}",
        plan.bonfire_session.bonfire_level,
        plan.bonfire_session.total_party_xp,
        plan.bonfire_session.flame_warmth_pct,
        plan.bonfire_session.pawshaw_trademark
    );
    for avatar in &plan.bonfire_session.avatars {
        println!(
            "  │   {:<14} {:<26} Lv.{:<3} — {}",
            avatar.emblem, avatar.handle, avatar.level, avatar.class_title
        );
    }
    println!("  └───────────────────────────────────────────────────────────────────────────────┘");
    println!();

    println!("  ┌── 6. STRIPE + APPLE PAY + GOOGLE PAY & MEM|16-10 WAVE RECEIPTS ───────────────┐");
    println!(
        "  │ Stripe Merchant : {} | Apple Pay: {} | Google Pay: {}",
        plan.payment_rails.stripe_merchant_id,
        plan.payment_rails.apple_pay_enabled,
        plan.payment_rails.google_pay_enabled
    );
    println!(
        "  │ Hearth Firewall : {}",
        plan.payment_rails.four_burner_hearth_firewall
    );
    println!(
        "  │ Wave Theory     : {:.2} Hz Carrier | {:.1} Hz Mom (*) | {:.1} Hz Dad (△) | [{}]",
        plan.wavechord.carrier_hz,
        plan.wavechord.overtone_mom_star_hz,
        plan.wavechord.bass_dad_pillar_hz,
        plan.wavechord.ascii_wave
    );
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
        "  │ FINAL COLLAPSE LEDGER HASH: 0x{:016x} | QUANT-TERN: {} (0x{:016x})",
        plan.final_collapse_hash,
        plan.quant_tern_receipt.qt_hex_tag,
        plan.quant_tern_receipt.golden_hash
    );
    println!("  └───────────────────────────────────────────────────────────────────────────────┘");
}
