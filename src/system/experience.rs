use bevy::prelude::*;

use crate::component::{
    experience::{ExperienceGem, PlayerStats},
    player::Player,
};

// Event to signal level up
#[derive(Event)]
pub struct LevelUpEvent;

pub fn collect_gems(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut PlayerStats), With<Player>>,
    gem_query: Query<(Entity, &Transform, &ExperienceGem)>,
    mut ev_level_up: EventWriter<LevelUpEvent>,
) {
    if let Ok((player_transform, mut player_stats)) = player_query.get_single_mut() {
        let magnet_radius = 150.0;
        let collect_radius = 30.0;

        for (gem_entity, gem_transform, gem) in &gem_query {
            let distance = player_transform
                .translation
                .distance(gem_transform.translation);

            // Move gem towards player if within magnet radius
            if distance < magnet_radius && distance > collect_radius {
                // Actually, to move it we need mut Transform on Gem, but for simplicity, we can do it in another system.
                // Let's just collect it instantly here if within collect_radius for now.
            }

            if distance < collect_radius {
                player_stats.current_xp += gem.amount;
                commands.entity(gem_entity).despawn();

                // Check for level up immediately
                if player_stats.current_xp >= player_stats.required_xp {
                    player_stats.level += 1;
                    player_stats.current_xp -= player_stats.required_xp;
                    player_stats.required_xp *= 1.5; // Scale next level requirement
                    ev_level_up.send(LevelUpEvent);
                    info!("Level Up! Now level {}", player_stats.level);
                }
            }
        }
    }
}
