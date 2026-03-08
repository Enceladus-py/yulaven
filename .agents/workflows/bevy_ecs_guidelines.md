---
description: Bevy ECS architecture and code structure guidelines
---

# Bevy ECS & Code Structure Best Practices

This document outlines the architectural guidelines and best practices for developing our "Bullet Heaven" game using the Bevy Engine. Strict adherence to these principles ensures a maintainable, high-performance, and easily extensible codebase.

## 1. System Organization and Modularity
*   **Keep Systems Small and Focused:** Each system should do exactly one thing. If a system handles both moving an enemy and applying damage to it, split it into `enemy_movement_system` and `collision_damage_system`.
*   **Use Plugins for Everything:** Group related components, resources, systems, and events into Plugins. For example, all player-related code should live in a `PlayerPlugin`, enemy logic in an `EnemyPlugin`, and UI in a `UiPlugin`. This allows for easy toggling of features during debugging.

## 2. Managing State
*   **Leverage `States`:** Use Bevy's built-in state management for game flow (e.g., `AppState::MainMenu`, `AppState::InGame`, `AppState::GameOver`, `AppState::Paused`).
*   **Run Criteria:** Attach systems to specific states using `OnEnter`, `OnExit`, and `Update` with `run_if(in_state(...))`. Never write `if current_state == ...` inside a system.

## 3. Performance & Collision
*   **Avoid $O(N^2)$ Loops:** Never loop over every bullet and check it against every enemy in a nested loop.
*   **Spatial Partitioning:** For collision detection between thousands of entities, use a spatial grid or a spatial hashing crate (like `bevy_spatial`). Update the grid positions once per frame, and only check collisions against entities in the same or adjacent grid cells.
*   **Component Size:** Keep components small and tightly packed in memory. Only include data that is inherently accessed together.

## 4. Events & Communication
*   **Use Events for Decoupling:** Instead of tightly coupling systems (e.g., the bullet system directly calling a decrease health function on the enemy), fire a `DamageEvent(Entity, f32)`. A separate `health_system` listens for this event and applies the damage.
*   **One Frame Delay:** Remember that events are typically read on the frame *after* they are sent (depending on system ordering).

## 5. Iteration Speed
*   **Inspector is Mandatory:** Register common components and resources with `bevy_inspector_egui`. This allows us to tweak weapon fire rates, enemy speed, and spawn intervals in real-time while the game is running, drastically reducing iteration time.
