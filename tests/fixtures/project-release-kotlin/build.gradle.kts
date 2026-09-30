plugins {
    id("com.android.application")
}

android {
    namespace = "com.example.doctorfixture"
    compileSdk = 36

    defaultConfig {
        applicationId = "com.example.doctorfixture"
        minSdk = 24
        targetSdk = 35
        versionCode = 7
        versionName = "1.2.3"
    }

    buildTypes {
        release {
            isDebuggable = false
        }
    }
}
