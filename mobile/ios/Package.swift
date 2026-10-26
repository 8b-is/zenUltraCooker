// swift-tools-version: 5.9
// ============================================================================
// zenUltraCooker v2.1.0 — Native iOS / iPadOS / macOS Swift Package
// Bundle ID : is.8b.zenultracooker
// SemVer    : 2.1.0 (CFBundleVersion: 20100)
// Merchant  : merchant.is.8b.zenultracooker (Stripe acct_1UUTcTRnidUxAlVy)
// ============================================================================

import PackageDescription

let package = Package(
    name: "ZenUltraCooker",
    platforms: [
        .iOS(.v16),
        .macOS(.v13)
    ],
    products: [
        .library(
            name: "ZenUltraCookerKit",
            targets: ["ZenUltraCookerKit"]
        ),
        .executable(
            name: "ZenUltraCookerApp",
            targets: ["ZenUltraCookerApp"]
        )
    ],
    targets: [
        .target(
            name: "ZenUltraCookerKit",
            path: "Sources/ZenUltraCookerKit"
        ),
        .executableTarget(
            name: "ZenUltraCookerApp",
            dependencies: ["ZenUltraCookerKit"],
            path: "Sources/ZenUltraCookerApp"
        )
    ]
)
