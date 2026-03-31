use bevy::prelude::*;

use super::components::{DamageFlash, Knockback};
use crate::{
    constant::{DAMAGE_FLASH_DURATION, ENEMY_KNOCKBACK_DURATION, ENEMY_KNOCKBACK_SPEED},
    core::components::Health,
    enemy::components::Enemy,
    player::character::SelectedCharacter,
    player::components::{Player, PlayerStats},
};

/// Warlock melee drain: damages nearby enemies within melee range periodically.
#[allow(clippy::type_complexity)]
pub fn warlock_melee_drain(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(
        &Transform,
        &mut Player,
        &PlayerStats,
        &crate::player::character::ActiveAbility,
    )>,
    mut enemy_query: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
) {
    let Ok((player_transform, mut player, stats, ability)) = player_query.single_mut() else {
        return;
    };

    if ability.kind != SelectedCharacter::Warlock {
        return;
    }

    player.orb_timer.tick(time.delta());
    if !player.orb_timer.just_finished() {
        return;
    }

    let melee_range = stats.attack_range;

    for (enemy_entity, enemy_transform, mut enemy_health) in &mut enemy_query {
        let distance = player_transform
            .translation
            .distance(enemy_transform.translation);

        if distance < melee_range {
            enemy_health.0 -= 20.0 * stats.damage_multiplier;

            commands.entity(enemy_entity).try_insert((
                DamageFlash(Timer::from_seconds(DAMAGE_FLASH_DURATION, TimerMode::Once)),
                Knockback {
                    velocity: (enemy_transform.translation - player_transform.translation)
                        .truncate()
                        .normalize_or_zero()
                        * ENEMY_KNOCKBACK_SPEED,
                    timer: Timer::from_seconds(ENEMY_KNOCKBACK_DURATION, TimerMode::Once),
                },
            ));
        }
    }
}

/// Warlock life drain passive: heals player on every enemy kill.
#[allow(clippy::type_complexity)]
pub fn warlock_life_drain(
    mut player_query: Query<
        (Entity, &mut Health),
        (With<crate::player::components::Player>, Without<Enemy>),
    >,
    enemy_query: Query<(Entity, &Health), With<Enemy>>,
    ability_query: Query<&crate::player::character::ActiveAbility>,
) {
    let Ok((player_entity, mut player_health)) = player_query.single_mut() else {
        return;
    };

    let Ok(ability) = ability_query.get(player_entity) else {
        return;
    };

    if ability.kind != SelectedCharacter::Warlock {
        return;
    }

    for (_enemy_entity, enemy_health) in &enemy_query {
        if enemy_health.0 <= 0.0 {
            player_health.0 = (player_health.0 + 1.0).min(150.0);
        }
    }
}
