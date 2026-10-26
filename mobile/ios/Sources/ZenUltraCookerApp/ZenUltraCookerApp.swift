import Foundation
#if canImport(ZenUltraCookerKit)
import ZenUltraCookerKit
#endif

#if canImport(SwiftUI) && !ZEN_CLI_SELF_TEST
import SwiftUI

@available(iOS 16.0, macOS 13.0, *)
public struct ZenUltraCookerRootView: View {
    @State private var store = SovereignHearthStore(zipCode: "44256")
    @State private var lastReceipt: StripeAppleGooglePayReceipt?

    public init() {}

    public var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 16) {
                // Header
                VStack(alignment: .leading, spacing: 6) {
                    Text("🍲 zenUltraCooker v\(ZenSemVer.versionString)")
                        .font(.title.bold())
                    Text(ZenSemVer.pawShawTrademark)
                        .font(.subheadline)
                        .foregroundStyle(.secondary)
                    Text("ZIP \(store.selectedZip.zipCode) · \(store.selectedZip.cityState) · \(store.selectedZip.storeFlyerName)")
                        .font(.caption)
                }
                .padding()
                .frame(maxWidth: .infinity, alignment: .leading)
                .background(Color.orange.opacity(0.14))
                .cornerRadius(14)

                // Appliances & Speakerphone
                VStack(alignment: .leading, spacing: 8) {
                    Text("🔥 Kitchen Appliances & Family Table")
                        .font(.headline)
                    Text("• Oven Status: BROKEN [−1] (Auto-rerouted to Air Fryer Ultra ★)")
                    Text("• Air Fryer Status: ULTRA APPLIANCE OF ALL TIME [+1 ★]")
                    Text("• Son in College on Speakerphone 🔊: \(store.collegeSonOnSpeakerphone ? "CONNECTED AT THE STOVE" : "STANDBY")")
                }
                .padding()
                .background(Color.green.opacity(0.12))
                .cornerRadius(12)

                // Honest Prep Recipes
                VStack(alignment: .leading, spacing: 8) {
                    Text("⏱️ Anti-Rachael Ray Honest Prep + Cook + Cleanup")
                        .font(.headline)
                    ForEach(store.recipes) { recipe in
                        VStack(alignment: .leading, spacing: 4) {
                            Text("\(recipe.title) (\(recipe.visibility))")
                                .font(.subheadline.bold())
                            Text("Prep: \(recipe.honestPrepMinutes)m · Cook: \(recipe.activeCookMinutes)m · Dishes: \(recipe.dishCleanupMinutes)m = \(recipe.totalRealWallClockMinutes)m real time")
                                .font(.caption)
                            Text("Route: \(store.routeRecipeForAppliances(recipe)) · \(recipe.starRating)★ (\(recipe.photoMomentsCount) 📸)")
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                        .padding(8)
                    }
                }
                .padding()
                .background(Color.blue.opacity(0.10))
                .cornerRadius(12)

                // Apple Pay / Google Pay / Stripe Checkout
                VStack(alignment: .leading, spacing: 10) {
                    Text("💳 1-Tap Family Cart (Stripe + Apple Pay + Google Pay)")
                        .font(.headline)
                    HStack(spacing: 10) {
                        Button(" Pay (\(String(format: "$%.2f", Double(store.checkoutTotalCents()) / 100.0)))") {
                            lastReceipt = store.simulateExpressPay(rail: "APPLE_PAY")
                        }
                        .buttonStyle(.borderedProminent)

                        Button("G Pay") {
                            lastReceipt = store.simulateExpressPay(rail: "GOOGLE_PAY")
                        }
                        .buttonStyle(.bordered)

                        Button("Stripe Link") {
                            lastReceipt = store.simulateExpressPay(rail: "STRIPE_LINK")
                        }
                        .buttonStyle(.bordered)
                    }
                    if let receipt = lastReceipt {
                        Text(receipt.eli5Summary)
                            .font(.caption)
                            .padding(8)
                            .background(Color.green.opacity(0.18))
                            .cornerRadius(8)
                    }
                }
                .padding()
                .background(Color.purple.opacity(0.10))
                .cornerRadius(12)
            }
            .padding()
        }
    }
}
#endif

@main
struct ZenUltraCookerCLIEntry {
    static func main() {
        let store = SovereignHearthStore(zipCode: "44256")
        precondition(ZenSemVer.versionString == "2.2.0", "SemVer mismatch")
        precondition(ZenSemVer.androidVersionCode == 20200, "versionCode mismatch")
        precondition(ZenSemVer.applePayMerchantId == "merchant.is.8b.zenultracooker", "Apple Pay Merchant ID mismatch")
        precondition(ZenSemVer.stripeAccountId == "acct_1UUTcTRnidUxAlVy", "Stripe Account ID mismatch")
        precondition(store.ovenState == .broken && store.airFryerState == .readyOrUltra, "Appliance state mismatch")
        let route = store.routeRecipeForAppliances(store.recipes[0])
        precondition(route.contains("AIR FRYER ULTRA"), "Broken oven auto-route failed")
        let receipt = store.simulateExpressPay(rail: "APPLE_PAY")
        precondition(receipt.hearthFirewallProtected && receipt.totalCents == 7340, "Receipt verification failed")

        print("zenUltraCooker iOS/macOS Native Target v\(ZenSemVer.versionString) (build \(ZenSemVer.androidVersionCode)) — SELF-TEST PASS (7/7 assertions)")
        print("Trademark: \(ZenSemVer.pawShawTrademark)")
        print("ZIP: \(store.selectedZip.zipCode) (\(store.selectedZip.cityState))")
        print("Receipt: \(receipt.eli5Summary)")
    }
}
