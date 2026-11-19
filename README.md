# Bonnie — Horizon 26.3

Bonfie port for Minecraft 26.3 / Horizon Rust Edition.

This repository contains the Horizon-native Rust mixin source. It is **not** a Fabric Java mod and must not be installed as a Java/Fabric mod JAR.

## Repository layout

- `horizon-native.json` — native bundle manifest. It pins the target game and Horizon/mixin revisions.
- `mixins/bonfie/horizon-mixin.toml` — Horizon mixin declaration.
- `mixins/bonfie/runtime.rs` — Bonfie's Rust state model and behavior helpers.

The native layer models Follow, Guard, Wander, Caretaker, Gather and Treasure modes, follow policies, mood/affection, work states, sitting/lying timers and interaction cooldowns.

## Important: fixing the `class_2378` startup crash

If the crash report contains both:

- `provided by 'bonfie' at 'ru.bonfiemod.BonfieMod'`
- `NoClassDefFoundError: net/minecraft/class_2378`

then the launcher is loading a legacy Java/Fabric Bonfie JAR. That Java entrypoint is incompatible with the current Minecraft 26.3 runtime. The Rust source in this repository does not define or execute `ru.bonfiemod.BonfieMod`.

To resolve this specific crash:

1. Close the game and launcher.
2. Open the Minecraft instance's `mods` directory.
3. Remove the old Bonfie/Bonfie-port Fabric JAR that contains the `ru.bonfiemod.BonfieMod` entrypoint. Do not remove unrelated mods.
4. Install/build this repository as a Horizon native bundle using the launcher's native-mixin workflow. Do not rename the native bundle to `.jar` or put the source folder in `mods`.
5. Launch Minecraft 26.3 again and verify that the crash report no longer lists the Java entrypoint `ru.bonfiemod.BonfieMod`.

Removing the incompatible Java JAR prevents this entrypoint crash; it does not by itself prove that the native bundle builds or that every Bonfie feature is implemented.

## Compatibility

The manifest currently targets:

- Minecraft: `26.3`
- Horizon revision: `baf0f96a7525841e01f28bbb7aea58d829dff11d`
- Mixin revision: `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`

Build and apply the native bundle against the exact Horizon revision installed by the launcher. A bundle built for a different Horizon revision may be rejected.

## Development

The state model has a unit test for the six-mode cycle. Run the crate's tests in the matching Horizon workspace after integrating the mixin. A successful Rust build alone does not verify Minecraft startup; test the resulting native bundle in a clean 26.3 instance without the legacy Java Bonfie JAR.
