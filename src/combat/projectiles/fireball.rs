use bevy::prelude::*;

use super::super::components::{CombatAssets, Fireball, FireballAnimation, Orb, Spell};
use crate::{
    constant::FIREBALL_SPEED_FACTOR,
    core::components::{DespawnNextFrame, MainCamera},
    enemy::components::Enemy,
    player::components::{Player, PlayerAnimation, PlayerStats},
};

const ORB_CHARGES_NEEDED: u8 = 5;
const MAX_PROJECTILES: usize = 40;

/// Returns true when `world_pos` is within the camera's visible world-space rectangle.
fn enemy_in_viewport(
    camera: &Camera,
    cam_gtf: &GlobalTransform,
    world_pos: Vec3,
    margin: f32,
) -> bool {
    let Some(viewport_size) = camera.logical_viewport_size() else {
        return true;
    };
    let corners = [
        Vec2::ZERO,
        Vec2::new(viewport_size.x, 0.0),
        Vec2::new(0.0, viewport_size.y),
        viewport_size,
    ];
    let mut min = Vec2::splat(f32::MAX);
    let mut max = Vec2::splat(f32::MIN);
    for corner in corners {
        if let Ok(ray) = camera.viewport_to_world(cam_gtf, corner) {
            min = min.min(ray.origin.truncate());
            max = max.max(ray.origin.truncate());
        }
    }
    let pos = world_pos.truncate();
    pos.x >= min.x - margin
        && pos.x <= max.x + margin
        && pos.y >= min.y - margin
        && pos.y <= max.y + margin
}

/// Finds the nearest enemy that is both within `max_range` and visible in viewport.
pub fn nearest_visible_enemy<'a>(
    origin: Vec3,
    enemy_query: &'a Query<&Transform, With<Enemy>>,
    camera: &Camera,
    cam_gtf: &GlobalTransform,
    max_range: f32,
) -> Option<&'a Transform> {
    enemy_query
        .iter()
        .filter(|t| {
            let dist = t.translation.distance(origin);
            dist <= max_range && enemy_in_viewport(camera, cam_gtf, t.translation, 60.0)
        })
        .min_by(|a, b| {
            let da = a.translation.distance_squared(origin);
            let db = b.translation.distance_squared(origin);
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
}

/// System for firing fireballs (Mage's charged attack).
#[allow(clippy::too_many_arguments)]
pub fn fire_fireballs(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(
        &Transform,
        &Sprite,
        &mut PlayerAnimation,
        &mut Player,
        &PlayerStats,
    )>,
    combat_assets: Res<CombatAssets>,
    enemy_query: Query<&Transform, With<Enemy>>,
    fireball_query: Query<Entity, With<Fireball>>,
    orb_query: Query<Entity, With<Orb>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Ok((camera, cam_gtf)) = camera_query.single() else {
        return;
    };
    if let Ok((player_transform, _sprite, mut animation, mut player, stats)) =
        player_query.single_mut()
    {
        player.fireball_timer.tick(time.delta());

        if !player.fireball_timer.just_finished() || player.orb_charges < ORB_CHARGES_NEEDED {
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

        player.orb_charges = 0;

        let player_pos = player_transform.translation.truncate();
        let enemy_pos = enemy_transform.translation.truncate();
        let direction: Vec2 = (enemy_pos - player_pos).normalize();
        let angle = direction.y.atan2(direction.x);
        let fireball_rotation = Quat::from_rotation_z(angle);

        let spawn_offset_dist = 30.0;
        let spawn_pos_vec2 = player_pos + (direction * spawn_offset_dist);
        let fireball_spawn_pos = spawn_pos_vec2.extend(player_transform.translation.z);

        if fireball_query.iter().count() + orb_query.iter().count() >= MAX_PROJECTILES {
            return;
        }

        animation.first_frame = 36;
        animation.last_frame = 46;
        animation
            .attack_timer
            .set_duration(std::time::Duration::from_secs_f32(0.4));
        animation.attack_timer.reset();

        commands.spawn((
            Sprite {
                image: combat_assets.projectile_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: combat_assets.atlas_layout.clone(),
                    index: 5,
                }),
                ..Default::default()
            },
            Transform {
                translation: fireball_spawn_pos,
                rotation: fireball_rotation,
                scale: Vec3::splat(4.0),
            },
            Fireball {
                progress: 0.0,
                direction,
            },
            FireballAnimation {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                first_frame: 5,
                last_frame: 6,
            },
            Spell {
                damage: 25.0 * stats.damage_multiplier,
            },
        ));
    }
}

/// System for moving fireballs.
pub fn move_fireballs(
    mut commands: Commands,
    mut fb_query: Query<(Entity, &mut Transform, &mut Fireball)>,
    pl_query: Query<&Transform, (With<Player>, Without<Fireball>)>,
    time: Res<Time>,
) {
    use crate::constant::FIREBALL_START_SPEED;
    for (fireball_entity, mut fb_transform, mut fb) in &mut fb_query {
        fb.progress += (time.delta_secs() * 2.0).clamp(0.0, 1.0);
        let speed = FIREBALL_START_SPEED + (fb.progress * FIREBALL_SPEED_FACTOR);

        fb_transform.translation += Vec3::from((fb.direction * speed * time.delta_secs(), 0.0));

        let Ok(pl_transform) = pl_query.single() else {
            continue;
        };
        if fb_transform.translation.distance(pl_transform.translation) > 1000.0 {
            commands.entity(fireball_entity).insert(DespawnNextFrame);
        }
    }
}
