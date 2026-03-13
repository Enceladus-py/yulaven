use bevy::{prelude::*, time::TimerMode};

use super::components::{Fireball, FireballAnimation, Invincible, Knockback, Orb, Spell};
use crate::{
    GameState,
    constant::{
        DAMAGE_FLASH_DURATION, ENEMY_KNOCKBACK_DURATION, ENEMY_KNOCKBACK_SPEED,
        FIREBALL_SPEED_FACTOR, FIREBALL_START_SPEED, INVINCIBILITY_DURATION, KNOCKBACK_DURATION,
        KNOCKBACK_SPEED, ORB_SPEED,
    },
    core::components::Health,
    enemy::components::{DamageFlash, Enemy},
    map::components::ExperienceGem,
    player::components::{Player, PlayerAnimation},
};

const ORB_CHARGES_NEEDED: u8 = 5;

pub fn fire_fireballs(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &Sprite, &mut PlayerAnimation, &mut Player)>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if let Ok((player_transform, sprite, mut animation, mut player)) = player_query.single_mut() {
        player.fireball_timer.tick(time.delta());

        if !player.fireball_timer.just_finished() || player.orb_charges < ORB_CHARGES_NEEDED {
            return;
        }
        player.orb_charges = 0;

        animation.first_frame = 36;
        animation.last_frame = 46;
        animation
            .attack_timer
            .set_duration(std::time::Duration::from_secs_f32(0.4));
        animation.attack_timer.reset();

        let Some(enemy_transform) = nearest_enemy(player_transform.translation, &enemy_query)
        else {
            return;
        };

        let texture_handle = asset_server.load("HumansProjectiles.png");
        let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 5, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let fb_offset_x = if sprite.flip_x { -20.0 } else { 20.0 };
        let direction = (enemy_transform.translation - player_transform.translation).truncate();
        let fireball_direction = direction.normalize_or_zero();
        let fireball_rotation =
            Quat::from_rotation_z(fireball_direction.y.atan2(fireball_direction.x));

        commands.spawn((
            Sprite {
                image: texture_handle.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_handle,
                    index: 5,
                }),
                ..Default::default()
            },
            Transform {
                translation: player_transform.translation
                    + Vec3 {
                        x: fb_offset_x,
                        y: 0.0,
                        z: 0.0,
                    },
                rotation: fireball_rotation,
                scale: Vec3::splat(4.0),
            },
            Fireball {
                progress: 0.0,
                direction: fireball_direction,
            },
            FireballAnimation {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                first_frame: 5,
                last_frame: 6,
            },
            Spell { damage: 25.0 },
        ));
    }
}

pub fn fire_orbs(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &Sprite, &mut PlayerAnimation, &mut Player)>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if let Ok((player_transform, sprite, mut animation, mut player)) = player_query.single_mut() {
        player.orb_timer.tick(time.delta());
        if !player.orb_timer.just_finished() {
            return;
        }

        animation.first_frame = 60;
        animation.last_frame = 64;
        animation
            .attack_timer
            .set_duration(std::time::Duration::from_secs_f32(0.3));
        animation.attack_timer.reset();

        player.orb_charges = player.orb_charges.saturating_add(1);

        let texture_handle = asset_server.load("HumansProjectiles.png");
        let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 5, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let offset_x = if sprite.flip_x { -20.0 } else { 20.0 };

        let initial_direction = nearest_enemy(player_transform.translation, &enemy_query).map_or(
            player.facing_direction,
            |et| {
                (et.translation - player_transform.translation)
                    .truncate()
                    .normalize_or_zero()
            },
        );

        commands.spawn((
            Sprite {
                image: texture_handle.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_handle,
                    index: 10,
                }),
                flip_x: sprite.flip_x,
                ..Default::default()
            },
            Transform {
                translation: player_transform.translation
                    + Vec3 {
                        x: offset_x,
                        y: 0.0,
                        z: 0.0,
                    },
                scale: Vec3::splat(4.0),
                ..Default::default()
            },
            Orb {
                direction: initial_direction,
            },
            Spell { damage: 15.0 },
        ));
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

        fb_transform.translation += Vec3::from((fb.direction * speed * time.delta_secs(), 0.0));

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

        orb.direction = orb.direction.lerp(steer_dir, 0.12).normalize_or_zero();
        orb_transform.translation += orb.direction.extend(0.0) * ORB_SPEED * time.delta_secs();

        if orb_transform.translation.distance(pl_transform.translation) > 1000.0 {
            commands.entity(orb_entity).despawn();
        }
    }
}

pub fn animate_spell(time: Res<Time>, mut fb_query: Query<(&mut Sprite, &mut FireballAnimation)>) {
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
                player_health.0 -= 1.0;

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
                    color: Color::srgb(0.2, 0.8, 0.2),
                    custom_size: Some(Vec2::new(15.0, 15.0)),
                    ..Default::default()
                },
                Transform::from_translation(transform.translation),
                ExperienceGem { amount: 10.0 },
            ));
        }
    }
}

fn nearest_enemy<'a>(
    origin: Vec3,
    enemy_query: &'a Query<&Transform, With<Enemy>>,
) -> Option<&'a Transform> {
    enemy_query.iter().min_by(|a, b| {
        let da = a.translation.distance_squared(origin);
        let db = b.translation.distance_squared(origin);
        da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
    })
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
