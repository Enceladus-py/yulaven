use bevy::prelude::*;

use super::components::{Player, PlayerAnimation};
use crate::map::components::{Collider, Structure};
use crate::ui::JoystickInput;
use crate::{constant::PLAYER_SPEED, core::components::MainCamera};

// Player movement system
pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    joystick: Res<JoystickInput>,
    mut player_query: Query<(&mut Transform, &mut PlayerAnimation, &mut Player), With<Player>>,
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

    if let Ok((mut player_transform, mut animation, mut player)) = player_query.single_mut() {
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
                + direction.normalize() * PLAYER_SPEED * time.delta_secs();

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
