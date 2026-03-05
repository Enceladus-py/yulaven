#![deny(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

use bevy::prelude::*;
use system::{
    animation::animate_sprite,
    combat::{handle_death, handle_enemy_player_collisions, handle_spell_collisions},
    enemy::{move_enemies, spawn_enemies, EnemySpawnTimer},
    experience::{collect_gems, LevelUpEvent},
    movement::{move_fireballs, move_orbs, move_player},
    spawn::{fire_fireballs, fire_orbs},
    startup::setup,
};

mod component;
mod constant;
mod system;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_event::<LevelUpEvent>()
        .init_resource::<EnemySpawnTimer>()
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player,
                move_fireballs,
                move_orbs,
                move_enemies,
                fire_fireballs,
                fire_orbs,
                spawn_enemies,
                animate_sprite,
                handle_spell_collisions,
                handle_enemy_player_collisions,
                handle_death,
                collect_gems,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}
