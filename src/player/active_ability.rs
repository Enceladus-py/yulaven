use bevy::prelude::*;

use super::character::{ActiveAbility, SelectedCharacter};
use super::components::{Player, PlayerStats};
use crate::combat::components::{Orb, Spell};
use crate::combat::nova::NovaEvent;
use crate::core::components::Health;
use crate::enemy::components::Enemy;
use crate::ui::JoystickInput;

/// Marker for the active ability UI button.
#[derive(Component)]
pub struct ActiveAbilityButton;

/// Reads Space bar (PC) / on-screen button inputs. Dispatches the correct
/// active ability based on the `ActiveAbility.kind` on the player.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn trigger_active_ability(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    joystick: Res<JoystickInput>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ActiveAbilityButton>)>,
    mut player_query: Query<(
        Entity,
        &mut Transform,
        &mut ActiveAbility,
        &mut Player,
        &PlayerStats,
        &mut Health,
    )>,
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform, &mut Health), (With<Enemy>, Without<Player>)>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut ev_nova: MessageWriter<NovaEvent>,
) {
    let Ok((player_entity, player_transform, mut ability, player, stats, mut player_health)) =
        player_query.single_mut()
    else {
        return;
    };

    ability.cooldown.tick(time.delta());

    if !ability.cooldown.is_finished() {
        return;
    }

    let mut pressed = keyboard.just_pressed(KeyCode::KeyQ);
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            pressed = true;
        }
    }

    if !pressed {
        return;
    }

    // Reset cooldown
    ability.cooldown.reset();

    match ability.kind {
        SelectedCharacter::Mage => {
            // Blink — teleport 320 px in the current facing / joystick direction
            let dir = if joystick.direction.length_squared() > 0.01 {
                joystick.direction.normalize()
            } else {
                player.facing_direction
            };

            let dist = 320.0;
            let target = player_transform.translation.truncate() + dir * dist;
            let mid_point = (player_transform.translation.truncate() + target) / 2.0;
            let angle = dir.y.atan2(dir.x);

            // Spawn the dash trail line
            commands.spawn((
                Sprite {
                    color: Color::srgba(0.2, 0.7, 1.0, 0.85),
                    custom_size: Some(Vec2::new(dist, 24.0)),
                    ..Default::default()
                },
                Transform {
                    translation: mid_point.extend(1.0), // render behind or above? let's do 1.0
                    rotation: Quat::from_rotation_z(angle),
                    ..Default::default()
                },
                crate::player::components::DashTrail {
                    lifetime: Timer::from_seconds(0.3, TimerMode::Once),
                },
            ));

            commands
                .entity(player_entity)
                .insert(crate::player::components::Teleporting {
                    timer: Timer::from_seconds(0.15, TimerMode::Once),
                    original_translation: player_transform.translation.truncate(),
                    target_translation: target,
                });
        }

        SelectedCharacter::Archer => {
            // Arrow Rain — fire 8 orbs evenly spaced around the player
            let texture_handle = asset_server.load("HumansProjectiles.png");
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 5, None, None);
            let layout_handle = texture_atlases.add(layout);

            for i in 0..8_u16 {
                let angle = std::f32::consts::TAU * (f32::from(i) / 8.0);
                let dir = Vec2::new(angle.cos(), angle.sin());
                commands.spawn((
                    Sprite {
                        image: texture_handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: layout_handle.clone(),
                            index: 10,
                        }),
                        ..Default::default()
                    },
                    Transform {
                        translation: player_transform.translation,
                        scale: Vec3::splat(4.0),
                        ..Default::default()
                    },
                    Orb { direction: dir },
                    Spell {
                        damage: 20.0 * stats.damage_multiplier,
                    },
                ));
            }
        }

        SelectedCharacter::Warlock => {
            // Void Nova — same AoE as Nova but also heals the Warlock
            ev_nova.write(NovaEvent {
                origin: player_transform.translation,
            });
            // Heal 8 HP (capped at max health)
            player_health.0 = (player_health.0 + 8.0).min(stats.max_health);
            let _ = (player_entity, enemy_query, commands); // suppress unused warnings
        }
    }
}

// HUD code moved to hud.rs
