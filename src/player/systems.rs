use bevy::prelude::*;

use super::components::{Player, PlayerAnimation, PlayerStats};
use crate::map::components::{Collider, Structure};
use crate::ui::JoystickInput;
use crate::{
    constant::{ARENA_RADIUS, PLAYER_SPEED},
    core::components::MainCamera,
};

// Player movement system
#[allow(clippy::type_complexity)]
pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    joystick: Res<JoystickInput>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut PlayerAnimation,
            &mut Player,
            &PlayerStats,
        ),
        (
            With<Player>,
            Without<crate::player::components::Teleporting>,
        ),
    >,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    structure_query: Query<(&GlobalTransform, &Collider), With<Structure>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;
    let mut facing_direction = Vec2::new(1.0, 0.0);
    let mut has_input = false;

    // --- Keyboard input ---
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
        facing_direction = Vec2::new(0.0, 1.0);
        has_input = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
        facing_direction = Vec2::new(0.0, -1.0);
        has_input = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
        facing_direction = Vec2::new(-1.0, 0.0);
        has_input = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
        facing_direction = Vec2::new(1.0, 0.0);
        has_input = true;
    }

    // --- Joystick / touch input (additive, overrides facing when active) ---
    if joystick.direction.length_squared() > 0.01 {
        direction += joystick.direction.extend(0.0);
        facing_direction = joystick.direction.normalize();
        has_input = true;
    }

    if let Ok((mut player_transform, mut animation, mut player, stats)) = player_query.single_mut()
    {
        // Subtle squash/stretch based on vertical movement.
        if direction.y > 0.0 {
            player_transform.scale.y = 4.3;
        } else if direction.y < 0.0 {
            player_transform.scale.y = 3.6;
        } else {
            player_transform.scale.y = 4.0;
        }

        if has_input && direction.length_squared() > 0.0 {
            // Normalize direction and move the player
            let mut new_translation = player_transform.translation
                + direction.normalize() * PLAYER_SPEED * stats.speed_multiplier * time.delta_secs();

            // Collision resolution
            for (str_transform, collider) in &structure_query {
                let diff = new_translation.truncate() - str_transform.translation().truncate();
                let dist_sq = diff.length_squared();
                let min_dist = 40.0 + collider.radius;

                if dist_sq < min_dist * min_dist {
                    let dist = dist_sq.sqrt();
                    let push_dir = if dist > 0.001 { diff / dist } else { Vec2::X };
                    let depth = min_dist - dist;
                    new_translation += (push_dir * depth).extend(0.0);
                }
            }

            // Keep the player inside the arena boundary
            if new_translation.truncate().length_squared() > ARENA_RADIUS * ARENA_RADIUS {
                new_translation = new_translation.truncate().normalize().extend(0.0) * ARENA_RADIUS;
            }

            player_transform.translation = new_translation;
            player.facing_direction = facing_direction;

            // Only change to running animation if not attacking
            if animation.attack_timer.is_finished() {
                animation.first_frame = 12;
                animation.last_frame = 17;
            }
        } else {
            // Only change to idle animation if not attacking
            if animation.attack_timer.is_finished() {
                animation.first_frame = 0;
                animation.last_frame = 3;
            }
        }

        // Update the camera's position to follow the player
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}

pub fn animate_player(
    time: Res<Time>,
    mut pl_query: Query<(&mut Sprite, &mut PlayerAnimation, &mut Player)>,
) {
    for (mut sprite, mut animation, player) in &mut pl_query {
        animation.timer.tick(time.delta());
        animation.attack_timer.tick(time.delta());
        if animation.timer.just_finished() {
            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index =
                    if atlas.index >= animation.last_frame || atlas.index < animation.first_frame {
                        animation.first_frame
                    } else {
                        atlas.index + 1
                    };
            }

            if player.facing_direction.x < -0.1 {
                sprite.flip_x = true;
            } else if player.facing_direction.x > 0.1 {
                sprite.flip_x = false;
            }
        }
    }
}

pub fn handle_teleportation(
    mut commands: Commands,
    mut pl_query: Query<(
        Entity,
        &mut Transform,
        &mut crate::player::components::Teleporting,
    )>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut teleporting) in &mut pl_query {
        teleporting.timer.tick(time.delta());
        let t = teleporting.timer.fraction();

        let new_pos = teleporting
            .original_translation
            .lerp(teleporting.target_translation, t);
        transform.translation.x = new_pos.x;
        // Keep the original z coordinate if it had one, though translation is Vec3
        transform.translation.y = new_pos.y;

        if teleporting.timer.just_finished() {
            commands
                .entity(entity)
                .remove::<crate::player::components::Teleporting>();
        }
    }
}

pub fn animate_dash_trail(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut crate::player::components::DashTrail,
        &mut Sprite,
    )>,
    time: Res<Time>,
) {
    for (entity, mut trail, mut sprite) in &mut query {
        trail.lifetime.tick(time.delta());

        let t = trail.lifetime.fraction(); // 0.0 to 1.0
        let alpha = (1.0 - t) * 0.85; // start at 0.85, fade out

        // Scale thickness down as it fades
        let thickness = 24.0 * (1.0 - t);
        if let Some(size) = sprite.custom_size.as_mut() {
            size.y = thickness;
        }

        sprite.color.set_alpha(alpha);

        if trail.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
