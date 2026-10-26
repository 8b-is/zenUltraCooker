plugins {
    id("com.android.application")
}

android {
    namespace = "is.eightb.zenultracooker"
    compileSdk = 36

    defaultConfig {
        applicationId = "is.eightb.zenultracooker"
        minSdk = 26
        targetSdk = 36
        // SemVer 2.1.0 -> MAJOR*10000 + MINOR*100 + PATCH = 20100
        versionCode = 20200
        versionName = "2.2.0"
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}
