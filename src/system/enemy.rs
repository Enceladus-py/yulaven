use bevy::prelude::*;
use rand::Rng;

use crate::component::{enemy::Enemy, health::Health, player::Player};

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

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    mut game_timer: ResMut<GameTimer>,
    player_query: Query<&Transform, With<Player>>,
) {
    game_timer.0.tick(time.delta());
    if game_timer.0.just_finished() {
        let current_duration = spawn_timer.0.duration().as_secs_f32();
        if current_duration > 0.2 {
            // Decrease spawn interval by 2% every second
            spawn_timer
                .0
                .set_duration(std::time::Duration::from_secs_f32(current_duration * 0.98));
        }
    }

    spawn_timer.0.tick(time.delta());
    if !spawn_timer.0.just_finished() {
        return;
    }
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let mut rng = rand::thread_rng();
    let angle: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
    let distance: f32 = 800.0; // Spawn outside view

    let spawn_pos = player_transform.translation
        + Vec3::new(angle.cos() * distance, angle.sin() * distance, 0.0);

    commands.spawn((
        Sprite {
            color: Color::srgb(0.6, 0.2, 0.2),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        Transform::from_translation(spawn_pos),
        Enemy,
        Health(30.0),
    ));
}

pub fn move_enemies(
    mut enemy_query: Query<(&mut Transform, &Enemy), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut enemy_transform, _enemy) in &mut enemy_query {
            let direction = (player_transform.translation - enemy_transform.translation).truncate();
            let distance = direction.length();
            if distance > 0.0 {
                let move_dir = direction.normalize();
                let speed = 50.0;
                enemy_transform.translation += move_dir.extend(0.0) * speed * time.delta_secs();
            }
        }
    }
}
