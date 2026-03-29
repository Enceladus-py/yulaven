use bevy::prelude::*;
use rand::Rng;

use super::components::{Enemy, EnemyKind, EnemyStats};
use crate::core::components::{DespawnNextFrame, Health};
use crate::player::components::Player;

const AGGRO_RADIUS: f32 = 700.0;
const MAX_ENEMIES: usize = 150;
const DESPAWN_RADIUS: f32 = 1400.0;

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

#[derive(Resource)]
pub struct GameTimer(pub Timer);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

impl Default for GameTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

/// Pick an enemy kind based on total elapsed game-seconds.
/// Early game = Grunts only; later waves unlock faster variants.
#[allow(clippy::cast_precision_loss)]
fn pick_enemy_kind(game_secs: f32, rng: &mut impl Rng) -> EnemyKind {
    // Build a weighted table that evolves over time.
    // Each entry is (EnemyKind, base_weight, unlocks_at_seconds).
    let candidates: &[(EnemyKind, f32, f32)] = &[
        (EnemyKind::Grunt, 40.0, 0.0),
        (EnemyKind::Goblin, 20.0, 15.0),
        (EnemyKind::Runner, 20.0, 30.0),
        (EnemyKind::Specter, 12.0, 60.0),
        (EnemyKind::Brute, 8.0, 90.0),
    ];

    let available: Vec<(EnemyKind, f32)> = candidates
        .iter()
        .filter(|(_, _, unlock)| game_secs >= *unlock)
        .map(|(kind, w, _)| (*kind, *w))
        .collect();

    let total: f32 = available.iter().map(|(_, w)| w).sum();
    let mut roll = rng.gen_range(0.0..total);
    for (kind, weight) in &available {
        roll -= weight;
        if roll <= 0.0 {
            return *kind;
        }
    }
    EnemyKind::Grunt
}

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    mut game_timer: ResMut<GameTimer>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    game_timer.0.tick(time.delta());
    let game_secs = game_timer.0.elapsed().as_secs_f32();

    if game_timer.0.just_finished() {
        let current_duration = spawn_timer.0.duration().as_secs_f32();
        if current_duration > 0.5 {
            // Decrease spawn interval by 2% every second, soft-capped at 0.5 s
            spawn_timer
                .0
                .set_duration(std::time::Duration::from_secs_f32(current_duration * 0.98));
        }
    }

    spawn_timer.0.tick(time.delta());
    if !spawn_timer.0.just_finished() {
        return;
    }

    let enemy_count = enemy_query.iter().count();
    log::debug!("enemy count: {enemy_count}");

    if enemy_count >= MAX_ENEMIES {
        return;
    }

    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let mut rng = rand::thread_rng();

    // Spawn 1–3 enemies per tick to create wave pressure at game_secs > 60
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let batch = if game_secs > 120.0 {
        3_usize
    } else if game_secs > 60.0 {
        2
    } else {
        1
    };

    for _ in 0..batch {
        if enemy_query.iter().count() >= MAX_ENEMIES {
            break;
        }
        let angle: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
        let distance: f32 = 850.0; // Spawn just outside the visible area

        let spawn_pos = player_transform.translation
            + Vec3::new(angle.cos() * distance, angle.sin() * distance, 0.0);

        let kind = pick_enemy_kind(game_secs, &mut rng);
        let health_val = kind.base_health();
        let sprite_size = kind.size();
        let color = kind.color();

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(sprite_size),
                ..Default::default()
            },
            Transform::from_translation(spawn_pos),
            Enemy { active: false },
            kind,
            EnemyStats {
                speed: kind.base_speed(),
                contact_damage: kind.contact_damage(),
                xp_drop: kind.xp_drop(),
            },
            Health(health_val),
        ));
    }
}

pub fn move_enemies(
    mut enemy_query: Query<(Entity, &mut Transform, &mut Enemy, &EnemyStats, &EnemyKind)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    structure_query: Query<
        (&GlobalTransform, &crate::map::components::Collider),
        With<crate::map::components::Structure>,
    >,
    time: Res<Time>,
    mut commands: Commands,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (entity, mut enemy_transform, mut enemy, stats, kind) in &mut enemy_query {
        let direction = (player_transform.translation - enemy_transform.translation).truncate();
        let distance = direction.length();

        // Activate within aggro radius, deactivate outside 1.2× (hysteresis)
        enemy.active = if distance < AGGRO_RADIUS {
            true
        } else if distance > AGGRO_RADIUS * 1.2 {
            false
        } else {
            enemy.active
        };

        if enemy.active && distance > 0.0 {
            let move_dir = direction.normalize();
            let mut new_translation = enemy_transform.translation
                + move_dir.extend(0.0) * stats.speed * time.delta_secs();

            // Specters phase through structures — skip collision for them
            if !kind.ignores_structures() {
                for (str_transform, collider) in &structure_query {
                    let diff = new_translation.truncate() - str_transform.translation().truncate();
                    let dist_sq = diff.length_squared();
                    let half_size = kind.size().x * 0.5;
                    let min_dist = half_size + collider.radius;

                    if dist_sq < min_dist * min_dist {
                        let dist = dist_sq.sqrt();
                        let push_dir = if dist > 0.001 { diff / dist } else { Vec2::X };
                        let depth = min_dist - dist;
                        new_translation += (push_dir * depth).extend(0.0);
                    }
                }
            }
            enemy_transform.translation = new_translation;
        }

        // Despawn if too far from player
        if distance > DESPAWN_RADIUS {
            commands.entity(entity).insert(DespawnNextFrame);
        }
    }
}
