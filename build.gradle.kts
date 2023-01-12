plugins {
    id("org.quiltmc.loom") version "1.+"
    id("io.github.p03w.machete") version "1.+"
    id("org.cadixdev.licenser") version "0.6.+"
    antlr
}

apply(from = "https://raw.githubusercontent.com/JamCoreModding/Gronk/quilt/publishing.gradle.kts")
apply(from = "https://raw.githubusercontent.com/JamCoreModding/Gronk/quilt/misc.gradle.kts")

val mod_version: String by project

group = "io.github.jamalam360"

version = mod_version

repositories {
    val mavenUrls =
        mapOf(
            Pair("https://maven.terraformersmc.com/releases", listOf("com.terraformersmc")),
            Pair("https://api.modrinth.com/maven", listOf("maven.modrinth")),
            Pair("https://maven.jamalam.tech/releases", listOf("io.github.jamalam360")),
        )

    for (mavenPair in mavenUrls) {
        maven {
            url = uri(mavenPair.key)
            content {
                for (group in mavenPair.value) {
                    includeGroup(group)
                }
            }
        }
    }

    mavenCentral()
}

dependencies {
    minecraft(libs.minecraft)
    mappings(variantOf(libs.quilt.mappings) { classifier("intermediary-v2") })

    modImplementation(libs.bundles.quilt)
    modApi(libs.bundles.required)
    modImplementation(libs.bundles.optional)
    modLocalRuntime(libs.bundles.runtime)

    antlr(libs.antlr)
}

sourceSets {
    val main = this.getByName("main")
    val language = create("language")

    main {
        this.compileClasspath += language.compileClasspath
        this.compileClasspath += language.output
        this.runtimeClasspath += language.runtimeClasspath
        this.runtimeClasspath += language.output
    }
}

