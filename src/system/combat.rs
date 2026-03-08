use bevy::{prelude::*, time::TimerMode};

use crate::{
    GameState,
    component::{
        enemy::{DamageFlash, Enemy},
        experience::ExperienceGem,
        health::Health,
        player::{Invincible, Knockback, Player},
        spell::Spell,
    },
    constant::{
        DAMAGE_FLASH_DURATION, ENEMY_KNOCKBACK_DURATION, ENEMY_KNOCKBACK_SPEED,
        INVINCIBILITY_DURATION, KNOCKBACK_DURATION, KNOCKBACK_SPEED,
    },
};

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
            // Simple distance check (~30 pixels)
            if distance < 30.0 {
                enemy_health.0 -= spell.damage;
                commands.entity(spell_entity).despawn();

                let knockback_dir = (enemy_transform.translation - spell_transform.translation)
                    .truncate()
                    .normalize_or_zero();

                commands.entity(enemy_entity).insert((
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

#[allow(clippy::type_complexity)]
pub fn handle_enemy_player_collisions(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &Transform, &mut Health),
        (With<Player>, Without<Enemy>, Without<Invincible>),
    >,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if let Ok((player_entity, player_transform, mut player_health)) = player_query.single_mut() {
        for enemy_transform in &enemy_query {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            if distance < 30.0 {
                player_health.0 -= 1.0; // Subtract some health

                let knockback_dir = (player_transform.translation - enemy_transform.translation)
                    .truncate()
                    .normalize_or_zero();

                commands.entity(player_entity).insert((
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

#[allow(clippy::type_complexity)]
pub fn handle_death(
    mut commands: Commands,
    query: Query<(
        Entity,
        &Health,
        Option<&Transform>,
        Option<&Enemy>,
        Option<&Player>,
    )>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity, health, opt_transform, opt_enemy, opt_player) in &query {
        if health.0 <= 0.0 {
            if opt_player.is_some() {
                next_state.set(GameState::GameOver);
                return;
            }

            commands.entity(entity).despawn();

            if opt_enemy.is_none() {
                continue;
            }
            let Some(transform) = opt_transform else {
                continue;
            };
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.2, 0.8, 0.2), // Green
                    custom_size: Some(Vec2::new(15.0, 15.0)),
                    ..Default::default()
                },
                Transform::from_translation(transform.translation),
                ExperienceGem { amount: 10.0 },
            ));
        }
    }
}
