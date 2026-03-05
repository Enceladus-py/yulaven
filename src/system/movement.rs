use bevy::prelude::*;

use crate::{
    component::{
        camera::MainCamera,
        fireball::Fireball,
        orb::Orb,
        player::{Player, PlayerAnimation, PlayerAttackMode},
    },
    constant::{FIREBALL_SPEED_FACTOR, FIREBALL_START_SPEED, ORB_SPEED, PLAYER_SPEED},
};

// Player movement system
pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut PlayerAnimation, &mut Player), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
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

    if let Ok((mut player_transform, mut animation, mut player)) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            player_transform.scale.y = 4.3; // Slight stretch when moving up
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            player_transform.scale.y = 3.6; // Slight squash when moving down
        } else {
            player_transform.scale.y = 4.0; // Reset when idle
        }

        if matches!(animation.attack_mode, PlayerAttackMode::None) {
            if direction.length_squared() > 0.0 {
                // Normalize direction and move the player
                player_transform.translation +=
                    direction.normalize() * PLAYER_SPEED * time.delta_secs();

                player.facing_direction = facing_direction;

                // Change to running animation
                animation.first_frame = 12;
                animation.last_frame = 17;
            } else {
                // Change to idle animation
                animation.first_frame = 0;
                animation.last_frame = 3;
            }
        }

        // Update the camera's position to follow the player
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
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
        if let Ok(pl_transform) = pl_query.get_single()
            && fb_transform.translation.distance(pl_transform.translation) > 1000.0
        {
            commands.entity(fireball_entity).despawn();
        }
    }
}

pub fn move_orbs(
    mut commands: Commands,
    mut orb_query: Query<(Entity, &mut Transform, &Sprite), With<Orb>>,
    pl_query: Query<&Transform, (With<Player>, Without<Orb>)>,
    time: Res<Time>,
) {
    if let Ok(pl_transform) = pl_query.get_single() {
        for (orb_entity, mut orb_transform, sprite) in &mut orb_query {
            orb_transform.translation.x = if sprite.flip_x {
                orb_transform.translation.x - ORB_SPEED * time.delta_secs()
            } else {
                orb_transform.translation.x + ORB_SPEED * time.delta_secs()
            };
            if orb_transform.translation.x > pl_transform.translation.x + 1000.0
                || orb_transform.translation.x < pl_transform.translation.x - 1000.0
            {
                commands.entity(orb_entity).despawn();
            }
        }
    }
}
