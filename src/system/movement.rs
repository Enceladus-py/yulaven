use bevy::prelude::*;

use crate::{
    component::{
        camera::MainCamera,
        enemy::Enemy,
        fireball::Fireball,
        orb::Orb,
        player::{Knockback, Player, PlayerAnimation},
    },
    constant::{FIREBALL_SPEED_FACTOR, FIREBALL_START_SPEED, ORB_SPEED, PLAYER_SPEED},
};

pub fn apply_knockback(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Knockback)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut knockback) in &mut query {
        knockback.timer.tick(time.delta());

        transform.translation += (knockback.velocity * time.delta_secs()).extend(0.0);

        if knockback.timer.is_finished() {
            commands.entity(entity).remove::<Knockback>();
        }
    }
}

// Player movement system
pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut PlayerAnimation, &mut Player), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    structure_query: Query<
        (&GlobalTransform, &crate::component::map::Collider),
        With<crate::component::map::Structure>,
    >,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;
    let mut facing_direction = Vec2::new(1.0, 0.0);

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
        facing_direction = Vec2::new(0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
        facing_direction = Vec2::new(0.0, -1.0);
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
        facing_direction = Vec2::new(-1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
        facing_direction = Vec2::new(1.0, 0.0);
    }

    if let Ok((mut player_transform, mut animation, mut player)) = player_query.single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            player_transform.scale.y = 4.3; // Slight stretch when moving up
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            player_transform.scale.y = 3.6; // Slight squash when moving down
        } else {
            player_transform.scale.y = 4.0; // Reset when idle
        }

        if direction.length_squared() > 0.0 {
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

pub fn move_fireballs(
    mut commands: Commands,
    mut fb_query: Query<(Entity, &mut Transform, &mut Fireball), With<Fireball>>,
    pl_query: Query<&Transform, (With<Player>, Without<Fireball>)>,
    time: Res<Time>,
) {
    for (fireball_entity, mut fb_transform, mut fb) in &mut fb_query {
        fb.progress += (time.delta_secs() * 2.0).clamp(0.0, 1.0);
        let speed = FIREBALL_START_SPEED + (fb.progress * FIREBALL_SPEED_FACTOR);

        // Move fireball based on velocity
        fb_transform.translation += Vec3::from((fb.direction * speed * time.delta_secs(), 0.0));

        // Despawn if fireball moves too far from the player
        let Ok(pl_transform) = pl_query.single() else {
            continue;
        };
        if fb_transform.translation.distance(pl_transform.translation) > 1000.0 {
            commands.entity(fireball_entity).despawn();
        }
    }
}

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
        // Steer toward nearest enemy; fall back to stored initial direction
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

        // Smoothly blend current direction toward target (simple lerp)
        orb.direction = orb.direction.lerp(steer_dir, 0.12).normalize_or_zero();

        orb_transform.translation += orb.direction.extend(0.0) * ORB_SPEED * time.delta_secs();

        // Despawn if orb drifts too far from player
        if orb_transform.translation.distance(pl_transform.translation) > 1000.0 {
            commands.entity(orb_entity).despawn();
        }
    }
}
