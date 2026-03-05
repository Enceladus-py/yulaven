use bevy::prelude::*;

use crate::component::{
    fireball::FireballAnimation,
    player::{Player, PlayerAnimation, PlayerAttackMode},
};

// System to animate sprite
pub fn animate_sprite(
    time: Res<Time>,
    mut pl_query: Query<(&mut Sprite, &mut PlayerAnimation, &mut Player)>,
    mut fb_query: Query<(&mut Sprite, &mut FireballAnimation), Without<Player>>,
) {
    for (mut sprite, mut animation, player) in &mut pl_query {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index =
                    if atlas.index >= animation.last_frame || atlas.index < animation.first_frame {
                        animation.first_frame
                    } else {
                        atlas.index + 1
                    };

                if matches!(
                    animation.attack_mode,
                    PlayerAttackMode::Orb | PlayerAttackMode::Fireball
                ) && atlas.index >= animation.last_frame
                {
                    animation.attack_mode = PlayerAttackMode::None;
                }
                println!("{}", atlas.index);
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
        if animation.timer.just_finished()
            && let Some(ref mut atlas) = sprite.texture_atlas
        {
            atlas.index =
                if atlas.index >= animation.last_frame || atlas.index < animation.first_frame {
                    animation.first_frame
                } else {
                    atlas.index + 1
                };
        }
    }
}
