use bevy::prelude::*;

use super::components::{Invincible, Knockback, Spell};
use crate::{
    constant::{
        DAMAGE_FLASH_DURATION, ENEMY_KNOCKBACK_DURATION, ENEMY_KNOCKBACK_SPEED,
        INVINCIBILITY_DURATION, KNOCKBACK_DURATION, KNOCKBACK_SPEED,
    },
    core::components::{DespawnNextFrame, Health},
    enemy::components::{DamageFlash, Enemy, EnemyStats},
    player::components::Player,
};

/// System for handling spell-enemy collisions.
#[allow(clippy::type_complexity)]
pub fn handle_spell_collisions(
    mut commands: Commands,
    spell_query: Query<(Entity, &Transform, &Spell), (Without<Enemy>, Without<Player>)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
) {
    for (spell_entity, spell_transform, spell) in &spell_query {
        for (enemy_entity, enemy_transform, mut enemy_health) in &mut enemy_query {
            let distance = spell_transform
                .translation
                .distance(enemy_transform.translation);

            if distance < 30.0 {
                enemy_health.0 -= spell.damage;
                commands.entity(spell_entity).insert(DespawnNextFrame);

                let knockback_dir = (enemy_transform.translation - spell_transform.translation)
                    .truncate()
                    .normalize_or_zero();

                commands.entity(enemy_entity).try_insert((
                    DamageFlash(Timer::from_seconds(DAMAGE_FLASH_DURATION, TimerMode::Once)),
                    Knockback {
                        velocity: knockback_dir * ENEMY_KNOCKBACK_SPEED,
                        timer: Timer::from_seconds(ENEMY_KNOCKBACK_DURATION, TimerMode::Once),
                    },
                ));

                break;
            }
        }
    }
}

/// System for handling enemy-player collisions.
#[allow(clippy::type_complexity)]
pub fn handle_enemy_player_collisions(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &Transform, &mut Health),
        (With<Player>, Without<Enemy>, Without<Invincible>),
    >,
    enemy_query: Query<(&Transform, &EnemyStats), With<Enemy>>,
) {
    if let Ok((player_entity, player_transform, mut player_health)) = player_query.single_mut() {
        for (enemy_transform, enemy_stats) in &enemy_query {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            if distance < 30.0 {
                player_health.0 -= enemy_stats.contact_damage;

                let knockback_dir = (player_transform.translation - enemy_transform.translation)
                    .truncate()
                    .normalize_or_zero();

                commands.entity(player_entity).try_insert((
                    Invincible(Timer::from_seconds(INVINCIBILITY_DURATION, TimerMode::Once)),
                    Knockback {
                        velocity: knockback_dir * KNOCKBACK_SPEED,
                        timer: Timer::from_seconds(KNOCKBACK_DURATION, TimerMode::Once),
                    },
                ));
                break;
            }
        }
    }
}
