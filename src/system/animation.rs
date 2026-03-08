use bevy::prelude::*;

use crate::component::{
    enemy::DamageFlash,
    fireball::FireballAnimation,
    player::{Invincible, Player, PlayerAnimation},
};

// System to animate sprite
pub fn animate_sprite(
    time: Res<Time>,
    mut pl_query: Query<(&mut Sprite, &mut PlayerAnimation, &mut Player)>,
    mut fb_query: Query<(&mut Sprite, &mut FireballAnimation), Without<Player>>,
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

            sprite.flip_x = if player.facing_direction == (Vec2 { x: -1.0, y: 0.0 }) {
                true
            } else if player.facing_direction == (Vec2 { x: 1.0, y: 0.0 }) {
                false
            } else {
                sprite.flip_x
            }
        }
    }

    for (mut sprite, mut animation) in &mut fb_query {
        animation.timer.tick(time.delta());
        if !animation.timer.just_finished() {
            continue;
        }
        let Some(ref mut atlas) = sprite.texture_atlas else {
            continue;
        };
        atlas.index = if atlas.index >= animation.last_frame || atlas.index < animation.first_frame
        {
            animation.first_frame
        } else {
            atlas.index + 1
        };
    }
}

pub fn handle_invincibility(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut Invincible)>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut invincible) in &mut query {
        invincible.0.tick(time.delta());

        if invincible.0.is_finished() {
            sprite.color.set_alpha(1.0);
            commands.entity(entity).remove::<Invincible>();
        } else {
            let t = invincible.0.elapsed_secs() * 10.0;
            let alpha = if t % 2.0 < 1.0 { 0.5 } else { 1.0 };
            sprite.color.set_alpha(alpha);
        }
    }
}

pub fn handle_damage_flash(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut DamageFlash)>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut flash) in &mut query {
        flash.0.tick(time.delta());

        if flash.0.is_finished() {
            sprite.color = Color::srgb(0.6, 0.2, 0.2); // Original enemy color
            commands.entity(entity).remove::<DamageFlash>();
        } else {
            sprite.color = Color::WHITE;
        }
    }
}
