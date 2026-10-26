import Foundation

// ============================================================================
// ZenUltraCookerKit v2.1.0 — Sovereign Family Hearth Core Model (iOS / macOS)
// Mirrors Rust `crates/zen-cooker-core` & Web `index.html` with strict SemVer.
// ============================================================================

public enum ZenSemVer {
    public static let versionString = "2.2.0"
    public static let major = 2
    public static let minor = 2
    public static let patch = 0
    public static let androidVersionCode = 20200
    public static let bundleIdentifier = "is.8b.zenultracooker"
    public static let applePayMerchantId = "merchant.is.8b.zenultracooker"
    public static let stripeAccountId = "acct_1UUTcTRnidUxAlVy"
    public static let pawShawTrademark = "PawShaw Kitchen Companion™"
}

public enum ApplianceState: Int, Codable, Sendable {
    case broken = -1
    case standby = 0
    case readyOrUltra = 1
}

public enum StorageZone: String, Codable, CaseIterable, Sendable {
    case fridge = "FRIDGE ❄️"
    case freezer = "FREEZER 🧊"
    case pantry = "PANTRY 🥫"
    case householdEssentials = "ESSENTIALS 🧻🐾"
}

public struct InventoryItem: Identifiable, Codable, Equatable, Sendable {
    public let id: String
    public var name: String
    public var zone: StorageZone
    public var quantityLabel: String
    public var daysUntilExpiry: Int?
    public var needsRestock: Bool
    public var unitCostCents: UInt32

    public init(
        id: String,
        name: String,
        zone: StorageZone,
        quantityLabel: String,
        daysUntilExpiry: Int?,
        needsRestock: Bool,
        unitCostCents: UInt32
    ) {
        self.id = id
        self.name = name
        self.zone = zone
        self.quantityLabel = quantityLabel
        self.daysUntilExpiry = daysUntilExpiry
        self.needsRestock = needsRestock
        self.unitCostCents = unitCostCents
    }
}

public struct RealPrepRecipe: Identifiable, Codable, Equatable, Sendable {
    public let id: String
    public var title: String
    public var requiresOvenByDefault: Bool
    public var airFryerCompatible: Bool
    public var honestPrepMinutes: UInt16
    public var activeCookMinutes: UInt16
    public var dishCleanupMinutes: UInt16
    public var visibility: String // "zenFamily 🌐" or "Family Secret 🔒"
    public var starRating: UInt8
    public var photoMomentsCount: UInt16
    public var costPerServingCents: UInt32

    public var totalRealWallClockMinutes: UInt16 {
        honestPrepMinutes + activeCookMinutes + dishCleanupMinutes
    }

    public var staysInWeeklyRotation: Bool {
        starRating >= 4
    }
}

public struct UsaZipMarketFeed: Codable, Equatable, Sendable {
    public let zipCode: String
    public let cityState: String
    public let storeFlyerName: String
    public let farmersMarketName: String
    public let weeklySavingsCents: UInt32
    public let doordashCancelSavingsCents: UInt32
}

public struct StripeAppleGooglePayReceipt: Codable, Equatable, Sendable {
    public let rail: String // "APPLE_PAY", "GOOGLE_PAY", "STRIPE_LINK"
    public let stripeAccountId: String
    public let merchantId: String
    public let zipCode: String
    public let totalCents: UInt32
    public let hearthFirewallProtected: Bool
    public let eli5Summary: String
}

public final class SovereignHearthStore {
    public var selectedZip: UsaZipMarketFeed
    public var ovenState: ApplianceState
    public var airFryerState: ApplianceState
    public var collegeSonOnSpeakerphone: Bool
    public var inventory: [InventoryItem]
    public var recipes: [RealPrepRecipe]
    public var quantAiNudges: [String]

    public static let supportedZips: [String: UsaZipMarketFeed] = [
        "44256": UsaZipMarketFeed(
            zipCode: "44256",
            cityState: "Medina, OH",
            storeFlyerName: "Buehler's Fresh Foods & ALDI Weekly Flyer",
            farmersMarketName: "Medina Square Saturday Farmers Market",
            weeklySavingsCents: 4650,
            doordashCancelSavingsCents: 6850
        ),
        "90210": UsaZipMarketFeed(
            zipCode: "90210",
            cityState: "Beverly Hills, CA",
            storeFlyerName: "Ralphs & Trader Joe's Weekly Ad",
            farmersMarketName: "Beverly Hills Sunday Civic Center Market",
            weeklySavingsCents: 5200,
            doordashCancelSavingsCents: 7400
        ),
        "78701": UsaZipMarketFeed(
            zipCode: "78701",
            cityState: "Austin, TX",
            storeFlyerName: "H-E-B Combo Locos Weekly Flyer",
            farmersMarketName: "SFC Farmers' Market Downtown",
            weeklySavingsCents: 4900,
            doordashCancelSavingsCents: 6600
        ),
        "10001": UsaZipMarketFeed(
            zipCode: "10001",
            cityState: "New York, NY",
            storeFlyerName: "Trader Joe's Chelsea & Whole Foods Weekly Deal",
            farmersMarketName: "Union Square Greenmarket",
            weeklySavingsCents: 5800,
            doordashCancelSavingsCents: 8200
        )
    ]

    public init(zipCode: String = "44256") {
        self.selectedZip = Self.supportedZips[zipCode] ?? Self.supportedZips["44256"]!
        self.ovenState = .broken
        self.airFryerState = .readyOrUltra
        self.collegeSonOnSpeakerphone = true
        self.inventory = [
            InventoryItem(id: "fridge-1", name: "Marinated Honey-Garlic Chicken Thighs", zone: .fridge, quantityLabel: "2.5 lbs", daysUntilExpiry: 1, needsRestock: false, unitCostCents: 699),
            InventoryItem(id: "fridge-2", name: "Baby Spinach & Sweet Peppers", zone: .fridge, quantityLabel: "1 bag", daysUntilExpiry: 2, needsRestock: false, unitCostCents: 299),
            InventoryItem(id: "freezer-1", name: "Frozen Broccoli Florets", zone: .freezer, quantityLabel: "2 bags", daysUntilExpiry: 90, needsRestock: false, unitCostCents: 249),
            InventoryItem(id: "pantry-1", name: "Jasmine Rice & Garlic Cloves", zone: .pantry, quantityLabel: "5 lb bag", daysUntilExpiry: 180, needsRestock: false, unitCostCents: 599),
            InventoryItem(id: "ess-tp", name: "Toilet Paper (12 Mega Rolls)", zone: .householdEssentials, quantityLabel: "2 rolls left", daysUntilExpiry: nil, needsRestock: true, unitCostCents: 999),
            InventoryItem(id: "ess-trash", name: "Tall Kitchen Trash Bags", zone: .householdEssentials, quantityLabel: "3 bags left", daysUntilExpiry: nil, needsRestock: true, unitCostCents: 749),
            InventoryItem(id: "ess-soap", name: "Free & Gentle Laundry Soap", zone: .householdEssentials, quantityLabel: "Low", daysUntilExpiry: nil, needsRestock: true, unitCostCents: 1199),
            InventoryItem(id: "ess-poop", name: "PawShaw Doggy Poop Bags ▽・ω・▽", zone: .householdEssentials, quantityLabel: "1 roll left", daysUntilExpiry: nil, needsRestock: true, unitCostCents: 599),
            InventoryItem(id: "ess-donuts", name: "Fresh Glazed Donuts (Advisor Lunch Tomorrow)", zone: .householdEssentials, quantityLabel: "1 dozen needed", daysUntilExpiry: 1, needsRestock: true, unitCostCents: 1299)
        ]
        self.recipes = [
            RealPrepRecipe(
                id: "rec-airfryer-thighs",
                title: "Dra's Crispy Air-Fryer Honey-Garlic Chicken & Broccoli",
                requiresOvenByDefault: true,
                airFryerCompatible: true,
                honestPrepMinutes: 7,
                activeCookMinutes: 14,
                dishCleanupMinutes: 4,
                visibility: "zenFamily 🌐",
                starRating: 5,
                photoMomentsCount: 19,
                costPerServingCents: 245
            ),
            RealPrepRecipe(
                id: "rec-sheetpan-fajitas",
                title: "15-Min DoorDash-Cancel Rescue Steak & Pepper Fajitas",
                requiresOvenByDefault: false,
                airFryerCompatible: true,
                honestPrepMinutes: 6,
                activeCookMinutes: 9,
                dishCleanupMinutes: 4,
                visibility: "Family Secret 🔒",
                starRating: 5,
                photoMomentsCount: 14,
                costPerServingCents: 280
            )
        ]
        self.quantAiNudges = [
            "🥩 Hey Dra! It's 4:15 PM — time to marinate the chicken thighs from the fridge (expires in 1 day, zero waste!).",
            "🍩 Hey! You have snacks tomorrow for advisor lunch — don't forget to grab the fresh donuts!",
            "🐾 Hey Chris / Nate! Please pick up dog poop bags for the dogs & trash bags on the way home.",
            "🔊 College Son is calling in on speakerphone at 6:15 PM — keep a warm plate by the stove!"
        ]
    }

    public func routeRecipeForAppliances(_ recipe: RealPrepRecipe) -> String {
        if recipe.requiresOvenByDefault && ovenState == .broken && airFryerState == .readyOrUltra && recipe.airFryerCompatible {
            return "AIR FRYER ULTRA ★ (Auto-rerouted from Broken Oven [−1])"
        }
        return "STANDARD STOVETOP / HEARTH [+1]"
    }

    public func checkoutTotalCents() -> UInt32 {
        let restockTotal = inventory.filter(\.needsRestock).reduce(UInt32(0)) { $0 + $1.unitCostCents }
        let dinnerIngredientsCents: UInt32 = 2495
        return restockTotal + dinnerIngredientsCents
    }

    public func simulateExpressPay(rail: String) -> StripeAppleGooglePayReceipt {
        let total = checkoutTotalCents()
        let formatted = String(format: "$%.2f", Double(total) / 100.0)
        return StripeAppleGooglePayReceipt(
            rail: rail,
            stripeAccountId: ZenSemVer.stripeAccountId,
            merchantId: ZenSemVer.applePayMerchantId,
            zipCode: selectedZip.zipCode,
            totalCents: total,
            hearthFirewallProtected: true,
            eli5Summary: "Paid \(formatted) via \(rail) → Stripe (\(ZenSemVer.stripeAccountId)). Burner 1 (Family Hearth) is 100% walled off from R&D (`Cap(Hearth) ⊥ Risk(R&D)`)."
        )
    }
}
