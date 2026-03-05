# YULAVEN

<p align="center">
  <img src="yulaven_pixel_logo.png" width="200" alt="Yulaven Pixel Logo">
</p>

> **"Where the soul-light flickers, the void hungers."**

**Yulaven** is a high-octane, arcane-survival action roguelike built with the **Bevy Engine**. Derived from the Old Turkic word *Yula* (soul/torch), Yulaven casts you as a mystic defender in a world consumed by an ancient, silent collapse.

## 🌌 The World of Yulaven

In the era of the *Yulaven* (the soul-haven), the boundaries between the physical realm and the primordial void have thinned. Legions of the *Harrowed*—creatures born from the static between stars—have begun their final harvest. As a Weaver of the Soul-Fire, you are the last light in a world going dark.

## ✨ Core Features

*   **Dynamic Arcane Combat**: Master the *Arcane Orbs*, which intelligently seek out the nearest threats, and unleash the devastating *Nova Fireball*—a high-impact spell charged by the very energy of your fallen foes.
*   **Intelligent AI Ecology**: Experience the *Aggro-Field* system, where enemies react based on proximity and threat level, creating tactical "pockets" of combat rather than an overwhelming, mindless swarm.
*   **Intuitive Soul-HUD**: Monitor your vital *Kut* (sacred energy) and experience progression through a sleek, minimal interface designed for high-stakes survival.
*   **Fluid Movement**: Cast and combat without restriction. Your mobility is your greatest weapon in the dance against the void.

## 🛠️ Project Architecture

Yulaven is built on a modular, ECS-driven architecture (Entity Component System) for maximum performance and scalability:

-   **`src/system/spawn.rs`**: The heart of the arcane logic, handling spell targeting and charge mechanics.
-   **`src/system/movement.rs`**: High-performance physics and steering logic for both projectiles and entities.
-   **`src/system/ui.rs`**: A reactive HUD system reflecting real-time game state.

## 🚀 Getting Started

Ensure you have the latest Rust toolchain installed.

```bash
# Clone and enter the void
git clone <repo-url>
cd yulaven

# Ignite the soul-torch
cargo run --release
```

---

*Yulaven is an ongoing exploration of mythic survival mechanics. Developed with passion and the power of Bevy.*