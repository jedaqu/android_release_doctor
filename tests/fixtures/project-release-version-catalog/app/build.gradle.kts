plugins {
    alias(libs.plugins.androidApplication)
}

android {
    namespace = "com.example.catalogfixture"
    compileSdk = 36

    defaultConfig {
        applicationId = "com.example.catalogfixture"
        minSdk = 24
        targetSdk = 35
        versionCode = 8
        versionName = "2.0.0"
    }

    buildTypes {
        release {
            isDebuggable = false
        }
    }
}
