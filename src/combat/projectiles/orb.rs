use bevy::prelude::*;

use super::super::components::{CombatAssets, Projectile, ProjectileKind, Spell};
use crate::{
    core::components::MainCamera,
    enemy::components::Enemy,
    player::character::SelectedCharacter,
    player::components::{Player, PlayerAnimation, PlayerStats},
};

use super::fireball::nearest_visible_enemy;

const MAX_PROJECTILES: usize = 40;
const ORB_SPEED: f32 = 300.0;
const ARROW_SPEED: f32 = 500.0;

/// Spawns a projectile toward the given direction.
#[allow(clippy::too_many_arguments)]
fn spawn_projectile(
    commands: &mut Commands,
    combat_assets: &CombatAssets,
    player_transform: &Transform,
    sprite: &Sprite,
    direction: Vec2,
    damage: f32,
    kind: ProjectileKind,
) {
    let offset_x = if sprite.flip_x { -20.0 } else { 20.0 };
    let (atlas_index, rotation) = match kind {
        ProjectileKind::Arrow => {
            let angle = direction.y.atan2(direction.x);
            (0, Quat::from_rotation_z(angle))
        }
        ProjectileKind::Orb => (10, Quat::IDENTITY),
    };

    commands.spawn((
        Sprite {
            image: combat_assets.projectile_image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: combat_assets.atlas_layout.clone(),
                index: atlas_index,
            }),
            flip_x: sprite.flip_x,
            ..Default::default()
        },
        Transform::from_translation(
            player_transform.translation
                + Vec3 {
                    x: offset_x,
                    y: 0.0,
                    z: 0.0,
                },
        )
        .with_rotation(rotation)
        .with_scale(Vec3::splat(4.0)),
        Projectile {
            direction,
            kind,
            speed: match kind {
                ProjectileKind::Orb => ORB_SPEED,
                ProjectileKind::Arrow => ARROW_SPEED,
            },
        },
        Spell { damage },
    ));
}

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
    projectile_query: Query<Entity, With<Projectile>>,
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

        if fireball_query.iter().count() + projectile_query.iter().count() >= MAX_PROJECTILES {
            return;
        }

        let initial_direction = (enemy_transform.translation - player_transform.translation)
            .truncate()
            .normalize_or_zero();

        animation.first_frame = 60;
        animation.last_frame = 64;
        animation
            .attack_timer
            .set_duration(std::time::Duration::from_secs_f32(0.3));
        animation.attack_timer.reset();

        match ability.kind {
            SelectedCharacter::Mage => {
                player.orb_charges = player.orb_charges.saturating_add(1);
                spawn_projectile(
                    &mut commands,
                    &combat_assets,
                    player_transform,
                    sprite,
                    initial_direction,
                    12.0 * stats.damage_multiplier,
                    ProjectileKind::Orb,
                );
            }
            SelectedCharacter::Archer => {
                spawn_projectile(
                    &mut commands,
                    &combat_assets,
                    player_transform,
                    sprite,
                    initial_direction,
                    12.0 * stats.damage_multiplier,
                    ProjectileKind::Arrow,
                );
            }
            SelectedCharacter::Warlock => {
                // Warlock: melee drain, no orbs
            }
        }
    }
}

/// System for moving all projectiles (orbs and arrows).
#[allow(clippy::type_complexity)]
pub fn move_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Transform, &mut Projectile)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Projectile>)>,
    pl_query: Query<&Transform, (With<Player>, Without<Projectile>)>,
    time: Res<Time>,
) {
    let Ok(pl_transform) = pl_query.single() else {
        return;
    };
    for (entity, mut transform, mut projectile) in &mut projectile_query {
        // Orbs home toward enemies, arrows fly straight
        if matches!(projectile.kind, ProjectileKind::Orb) {
            let steer_dir = enemy_query
                .iter()
                .min_by(|a, b| {
                    let da = a.translation.distance_squared(transform.translation);
                    let db = b.translation.distance_squared(transform.translation);
                    da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
                })
                .map_or(projectile.direction, |et| {
                    (et.translation - transform.translation)
                        .truncate()
                        .normalize_or_zero()
                });

            projectile.direction = projectile
                .direction
                .lerp(steer_dir, 0.12)
                .normalize_or_zero();
        }

        transform.translation +=
            projectile.direction.extend(0.0) * projectile.speed * time.delta_secs();

        if transform.translation.distance(pl_transform.translation) > 1000.0 {
            commands
                .entity(entity)
                .insert(crate::core::components::DespawnNextFrame);
        }
    }
}
