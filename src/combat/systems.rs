use bevy::{prelude::*, time::TimerMode};

use super::components::{
    CombatAssets, Fireball, FireballAnimation, Invincible, Knockback, Orb, Spell,
};
use crate::{
    GameState,
    constant::{
        DAMAGE_FLASH_DURATION, ENEMY_KNOCKBACK_DURATION, ENEMY_KNOCKBACK_SPEED,
        FIREBALL_SPEED_FACTOR, FIREBALL_START_SPEED, INVINCIBILITY_DURATION, KNOCKBACK_DURATION,
        KNOCKBACK_SPEED, ORB_SPEED,
    },
    core::components::{DespawnNextFrame, Health, MainCamera},
    enemy::components::{DamageFlash, Enemy, EnemyKind, EnemyStats},
    map::components::ExperienceGem,
    player::components::{Player, PlayerAnimation, PlayerStats},
};

const ORB_CHARGES_NEEDED: u8 = 5;
const MAX_GEMS: usize = 300; // Limit gems to prevent Mali driver crashes on Android
const MAX_PROJECTILES: usize = 40; // Cap projectiles to prevent Mali driver overload on rapid fire

/// Returns true when `world_pos` is within the camera's visible world-space rectangle.
/// A `margin` (in world units) can extend the check slightly beyond the visible edge so
/// enemies just entering the screen feel natural to target immediately.
fn enemy_in_viewport(
    camera: &Camera,
    cam_gtf: &GlobalTransform,
    world_pos: Vec3,
    margin: f32,
) -> bool {
    // viewport_to_world gives us the world-space ray for a viewport point.
    // We reconstruct the world-space AABB from the four corners of the viewport.
    let Some(viewport_size) = camera.logical_viewport_size() else {
        return true; // fallback: allow firing if we can't determine viewport
    };
    let corners = [
        Vec2::ZERO,
        Vec2::new(viewport_size.x, 0.0),
        Vec2::new(0.0, viewport_size.y),
        viewport_size,
    ];
    let mut min = Vec2::splat(f32::MAX);
    let mut max = Vec2::splat(f32::MIN);
    for corner in corners {
        if let Ok(ray) = camera.viewport_to_world(cam_gtf, corner) {
            min = min.min(ray.origin.truncate());
            max = max.max(ray.origin.truncate());
        }
    }
    let pos = world_pos.truncate();
    pos.x >= min.x - margin
        && pos.x <= max.x + margin
        && pos.y >= min.y - margin
        && pos.y <= max.y + margin
}

/// Pre-loads the projectile texture + atlas layout ONCE when entering Playing state.
/// This prevents `TextureAtlasLayout::from_grid` + `texture_atlases.add` from being
/// called on every shot, which caused `FlushedStagingBuffer::drop` crashes in the Mali driver.
pub fn setup_combat_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image = asset_server.load("HumansProjectiles.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 5, None, None);
    let layout_handle = texture_atlases.add(layout);
    commands.insert_resource(CombatAssets {
        projectile_image: image,
        atlas_layout: layout_handle,
    });
}

#[allow(clippy::too_many_arguments)]
pub fn fire_fireballs(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(
        &Transform,
        &Sprite,
        &mut PlayerAnimation,
        &mut Player,
        &PlayerStats,
    )>,
    combat_assets: Res<CombatAssets>,
    enemy_query: Query<&Transform, With<Enemy>>,
    fireball_query: Query<Entity, With<Fireball>>,
    orb_query: Query<Entity, With<Orb>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Ok((camera, cam_gtf)) = camera_query.single() else {
        return;
    };
    if let Ok((player_transform, _sprite, mut animation, mut player, stats)) =
        player_query.single_mut()
    {
        player.fireball_timer.tick(time.delta());

        if !player.fireball_timer.just_finished() || player.orb_charges < ORB_CHARGES_NEEDED {
            return;
        }

        let Some(enemy_transform) = nearest_visible_enemy(
            player_transform.translation,
            &enemy_query,
            camera,
            cam_gtf,
            stats.attack_range,
        ) else {
            return;
        };

        player.orb_charges = 0;

        // 1. Convert Vec3 translations to Vec2
        let player_pos = player_transform.translation.truncate();
        let enemy_pos = enemy_transform.translation.truncate();

        // 2. Now the math works because both are Vec2
        let direction: Vec2 = (enemy_pos - player_pos).normalize();

        // 3. Calculate rotation using the Vec2 components
        let angle = direction.y.atan2(direction.x);
        let fireball_rotation = Quat::from_rotation_z(angle);

        // 4. For the spawn position, you can convert back to Vec3
        let spawn_offset_dist = 30.0;
        let spawn_pos_vec2 = player_pos + (direction * spawn_offset_dist);
        let fireball_spawn_pos = spawn_pos_vec2.extend(player_transform.translation.z);

        // Don't spawn more if too many projectiles already on screen
        if fireball_query.iter().count() + orb_query.iter().count() >= MAX_PROJECTILES {
            return;
        }

        animation.first_frame = 36;
        animation.last_frame = 46;
        animation
            .attack_timer
            .set_duration(std::time::Duration::from_secs_f32(0.4));
        animation.attack_timer.reset();

        commands.spawn((
            Sprite {
                image: combat_assets.projectile_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: combat_assets.atlas_layout.clone(),
                    index: 5,
                }),
                ..Default::default()
            },
            Transform {
                translation: fireball_spawn_pos,
                rotation: fireball_rotation,
                scale: Vec3::splat(4.0),
            },
            Fireball {
                progress: 0.0,
                direction,
            },
            FireballAnimation {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                first_frame: 5,
                last_frame: 6,
            },
            Spell {
                damage: 25.0 * stats.damage_multiplier,
            },
        ));
    }
}

/// Fires orbs for characters that use orb-based attacks.
///
/// `Mage`: orbs charge up fireballs (handled in `fire_fireballs`)
/// `Archer`: rapid auto-orbs targeting nearby enemies
#[allow(clippy::too_many_arguments)]
pub fn fire_orbs(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(
        &Transform,
        &Sprite,
        &mut PlayerAnimation,
        &mut Player,
        &PlayerStats,
        &crate::player::character::ActiveAbility,
    )>,
    combat_assets: Res<CombatAssets>,
    enemy_query: Query<&Transform, With<Enemy>>,
    fireball_query: Query<Entity, With<Fireball>>,
    orb_query: Query<Entity, With<Orb>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Ok((camera, cam_gtf)) = camera_query.single() else {
        return;
    };
    if let Ok((player_transform, sprite, mut animation, mut player, stats, ability)) =
        player_query.single_mut()
    {
        player.orb_timer.tick(time.delta());
        if !player.orb_timer.just_finished() {
            return;
        }

        let Some(enemy_transform) = nearest_visible_enemy(
            player_transform.translation,
            &enemy_query,
            camera,
            cam_gtf,
            stats.attack_range,
        ) else {
            return;
        };

        // Don't spawn more if too many projectiles already on screen
        if fireball_query.iter().count() + orb_query.iter().count() >= MAX_PROJECTILES {
            return;
        }

        // Character-specific behavior
        match ability.kind {
            crate::player::character::SelectedCharacter::Mage => {
                // Mage: orb charges fireball, not direct orb attack
                player.orb_charges = player.orb_charges.saturating_add(1);
                animation.first_frame = 60;
                animation.last_frame = 64;
                animation
                    .attack_timer
                    .set_duration(std::time::Duration::from_secs_f32(0.3));
                animation.attack_timer.reset();
            }
            crate::player::character::SelectedCharacter::Archer => {
                // Archer: rapid auto-orbs targeting nearby enemies
                animation.first_frame = 60;
                animation.last_frame = 64;
                animation
                    .attack_timer
                    .set_duration(std::time::Duration::from_secs_f32(0.3));
                animation.attack_timer.reset();

                let offset_x = if sprite.flip_x { -20.0 } else { 20.0 };
                let initial_direction = (enemy_transform.translation
                    - player_transform.translation)
                    .truncate()
                    .normalize_or_zero();

                commands.spawn((
                    Sprite {
                        image: combat_assets.projectile_image.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: combat_assets.atlas_layout.clone(),
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
                    Spell {
                        damage: 12.0 * stats.damage_multiplier,
                    },
                ));
            }
            crate::player::character::SelectedCharacter::Warlock => {
                // Warlock: melee drain, no orbs
            }
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

        fb_transform.translation += Vec3::from((fb.direction * speed * time.delta_secs(), 0.0));

        let Ok(pl_transform) = pl_query.single() else {
            continue;
        };
        if fb_transform.translation.distance(pl_transform.translation) > 1000.0 {
            commands.entity(fireball_entity).insert(DespawnNextFrame);
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
            commands.entity(orb_entity).insert(DespawnNextFrame);
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

            // Limit gem count to prevent GPU driver overcrowding
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

/// Finds the nearest enemy that is both within `max_range` world units of `origin`
/// AND visible within the camera's viewport (with a 60px margin).
fn nearest_visible_enemy<'a>(
    origin: Vec3,
    enemy_query: &'a Query<&Transform, With<Enemy>>,
    camera: &Camera,
    cam_gtf: &GlobalTransform,
    max_range: f32,
) -> Option<&'a Transform> {
    enemy_query
        .iter()
        .filter(|t| {
            let dist = t.translation.distance(origin);
            dist <= max_range && enemy_in_viewport(camera, cam_gtf, t.translation, 60.0)
        })
        .min_by(|a, b| {
            let da = a.translation.distance_squared(origin);
            let db = b.translation.distance_squared(origin);
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
}

/// Warlock melee drain: damages nearby enemies within melee range periodically
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

    // Only for Warlock
    if ability.kind != crate::player::character::SelectedCharacter::Warlock {
        return;
    }

    player.orb_timer.tick(time.delta());
    if !player.orb_timer.just_finished() {
        return;
    }

    let melee_range = stats.attack_range; // 80 for Warlock

    for (enemy_entity, enemy_transform, mut enemy_health) in &mut enemy_query {
        let distance = player_transform
            .translation
            .distance(enemy_transform.translation);

        if distance < melee_range {
            enemy_health.0 -= 20.0 * stats.damage_multiplier;

            // Visual feedback: damage flash
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

/// Warlock life drain passive: heals player on every enemy kill
#[allow(clippy::type_complexity)]
pub fn warlock_life_drain(
    _commands: Commands,
    _query: Query<(Entity, &Health, Option<&Player>)>,
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

    // Only for Warlock
    if ability.kind != crate::player::character::SelectedCharacter::Warlock {
        return;
    }

    // Check for dead enemies and heal before they despawn
    for (_enemy_entity, enemy_health) in &enemy_query {
        if enemy_health.0 <= 0.0 {
            // Heal the player (life drain: 1 HP per kill)
            player_health.0 = (player_health.0 + 1.0).min(150.0); // Cap at Warlock's max health
        }
    }
}

pub fn handle_damage_flash(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut DamageFlash, Option<&EnemyKind>)>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut flash, opt_kind) in &mut query {
        flash.0.tick(time.delta());

        if flash.0.is_finished() {
            // Restore to the kind-specific original color (or white for non-enemies)
            sprite.color = opt_kind.map_or(Color::WHITE, |k| k.color());
            commands.entity(entity).remove::<DamageFlash>();
        } else {
            sprite.color = Color::WHITE;
        }
    }
}
