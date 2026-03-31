use bevy::prelude::*;

use super::super::components::{CombatAssets, Orb, Spell};
use crate::{
    core::components::MainCamera,
    enemy::components::Enemy,
    player::character::SelectedCharacter,
    player::components::{Player, PlayerAnimation, PlayerStats},
};

use super::fireball::nearest_visible_enemy;

const MAX_PROJECTILES: usize = 40;
const ORB_SPEED: f32 = 300.0;

/// System for firing orbs (Archer's rapid attack, Mage's charge mechanic).
#[allow(clippy::too_many_arguments)]
pub fn fire_orbs(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(
        &Transform,
        &Sprite,
        &mut PlayerAnimation,
        &mut Player,
        &PlayerStats,
        &crate::player::character::ActiveAbility,
    )>,
    combat_assets: Res<CombatAssets>,
    enemy_query: Query<&Transform, With<Enemy>>,
    fireball_query: Query<Entity, With<super::super::components::Fireball>>,
    orb_query: Query<Entity, With<Orb>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Ok((camera, cam_gtf)) = camera_query.single() else {
        return;
    };
    if let Ok((player_transform, sprite, mut animation, mut player, stats, ability)) =
        player_query.single_mut()
    {
        player.orb_timer.tick(time.delta());
        if !player.orb_timer.just_finished() {
            return;
        }

        let Some(enemy_transform) = nearest_visible_enemy(
            player_transform.translation,
            &enemy_query,
            camera,
            cam_gtf,
            stats.attack_range,
        ) else {
            return;
        };

        if fireball_query.iter().count() + orb_query.iter().count() >= MAX_PROJECTILES {
            return;
        }

        match ability.kind {
            SelectedCharacter::Mage => {
                player.orb_charges = player.orb_charges.saturating_add(1);
                animation.first_frame = 60;
                animation.last_frame = 64;
                animation
                    .attack_timer
                    .set_duration(std::time::Duration::from_secs_f32(0.3));
                animation.attack_timer.reset();
            }
            SelectedCharacter::Archer => {
                animation.first_frame = 60;
                animation.last_frame = 64;
                animation
                    .attack_timer
                    .set_duration(std::time::Duration::from_secs_f32(0.3));
                animation.attack_timer.reset();

                let offset_x = if sprite.flip_x { -20.0 } else { 20.0 };
                let initial_direction = (enemy_transform.translation
                    - player_transform.translation)
                    .truncate()
                    .normalize_or_zero();

                commands.spawn((
                    Sprite {
                        image: combat_assets.projectile_image.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: combat_assets.atlas_layout.clone(),
                            index: 10,
                        }),
                        flip_x: sprite.flip_x,
                        ..Default::default()
                    },
                    Transform {
                        translation: player_transform.translation
                            + Vec3 {
                                x: offset_x,
                                y: 0.0,
                                z: 0.0,
                            },
                        scale: Vec3::splat(4.0),
                        ..Default::default()
                    },
                    Orb {
                        direction: initial_direction,
                    },
                    Spell {
                        damage: 12.0 * stats.damage_multiplier,
                    },
                ));
            }
            SelectedCharacter::Warlock => {
                // Warlock: melee drain, no orbs
            }
        }
    }
}

/// System for moving orbs with homing behavior.
#[allow(clippy::type_complexity)]
pub fn move_orbs(
    mut commands: Commands,
    mut orb_query: Query<(Entity, &mut Transform, &mut Orb)>,
    pl_query: Query<&Transform, (With<Player>, Without<Orb>)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Orb>, Without<Player>)>,
    time: Res<Time>,
) {
    let Ok(pl_transform) = pl_query.single() else {
        return;
    };
    for (orb_entity, mut orb_transform, mut orb) in &mut orb_query {
        let steer_dir = enemy_query
            .iter()
            .min_by(|a, b| {
                let da = a.translation.distance_squared(orb_transform.translation);
                let db = b.translation.distance_squared(orb_transform.translation);
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map_or(orb.direction, |et| {
                (et.translation - orb_transform.translation)
                    .truncate()
                    .normalize_or_zero()
            });

        orb.direction = orb.direction.lerp(steer_dir, 0.12).normalize_or_zero();
        orb_transform.translation += orb.direction.extend(0.0) * ORB_SPEED * time.delta_secs();

        if orb_transform.translation.distance(pl_transform.translation) > 1000.0 {
            commands
                .entity(orb_entity)
                .insert(crate::core::components::DespawnNextFrame);
        }
    }
}
