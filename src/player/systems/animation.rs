use bevy::prelude::*;

use super::super::components::{Player, PlayerAnimation};

/// System for animating the player sprite based on animation timers.
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
