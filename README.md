# Bonnie — Horizon 26.3

Rust/Horizon 26.3 port of the supplied Bonfie Fabric mod.

## Contents

- `bonfie-horizon-26.3-port.zip` — complete port source archive with carried assets and the native Rust layer.
- The native layer implements Bonfie's state machine: Follow, Guard, Wander, Caretaker, Gather and Treasure; follow policies; mood/affection; work states; sit/lie state and interaction cooldowns.

## Important

The original mod is a Fabric/Java 1.20.1 mod. Horizon does not execute Fabric Java classes directly, so Java behavior must be implemented through Horizon's native Rust API.

The final native bundle must be built against the exact Horizon revision installed by the launcher. A native bundle built for another Horizon revision may be rejected by the launcher.

## Target

Minecraft 26.3 / Horizon Rust Edition.
