use bevy::prelude::*;

use super::components::ExperienceGem;
use crate::{
    GameState,
    core::components::{DespawnNextFrame, Health},
    enemy::components::{Enemy, EnemyStats},
    player::components::Player,
};

const MAX_GEMS: usize = 300;

/// System for handling enemy death and experience gem drops.
#[allow(clippy::type_complexity)]
pub fn handle_death(
    mut commands: Commands,
    query: Query<(
        Entity,
        &Health,
        Option<&Transform>,
        Option<&Enemy>,
        Option<&EnemyStats>,
        Option<&Player>,
    )>,
    gem_query: Query<Entity, With<ExperienceGem>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let gem_count = gem_query.iter().count();

    for (entity, health, opt_transform, opt_enemy, opt_stats, opt_player) in &query {
        if health.0 <= 0.0 {
            if opt_player.is_some() {
                next_state.set(GameState::GameOver);
                return;
            }

            commands.entity(entity).insert(DespawnNextFrame);

            if opt_enemy.is_none() {
                continue;
            }
            let Some(transform) = opt_transform else {
                continue;
            };

            let xp = opt_stats.map_or(10.0, |s| s.xp_drop);

            if gem_count < MAX_GEMS {
                commands.spawn((
                    Sprite {
                        color: Color::srgb(0.2, 0.8, 0.2),
                        custom_size: Some(Vec2::new(15.0, 15.0)),
                        ..Default::default()
                    },
                    Transform::from_translation(transform.translation),
                    ExperienceGem { amount: xp },
                ));
            }
        }
    }
}
