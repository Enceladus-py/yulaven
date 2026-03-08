#![deny(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

use bevy::prelude::*;
use system::{
    animation::{animate_sprite, handle_damage_flash, handle_invincibility},
    combat::{handle_death, handle_enemy_player_collisions, handle_spell_collisions},
    enemy::{EnemySpawnTimer, GameTimer, move_enemies, spawn_enemies},
    experience::{LevelUpEvent, collect_gems},
    map::update_terrain,
    movement::{apply_knockback, move_fireballs, move_orbs, move_player},
    spawn::{fire_fireballs, fire_orbs},
    startup::setup,
    ui::{
        handle_restart, handle_skill_selection, spawn_gameover_menu, spawn_hud, spawn_large_map,
        spawn_levelup_menu, spawn_minimap_hud, spawn_weapon_hud, toggle_map, transition_to_levelup,
        update_hud, update_map, update_weapon_hud,
    },
};

mod component;
mod constant;
mod system;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Playing,
    LevelUp,
    Paused,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_message::<LevelUpEvent>()
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<GameTimer>()
        .init_state::<GameState>()
        .add_systems(
            Startup,
            (
                setup,
                spawn_hud,
                spawn_weapon_hud,
                spawn_minimap_hud,
                spawn_large_map,
            ),
        )
        .add_systems(
            Update,
            (
                move_player,
                apply_knockback,
                move_fireballs,
                move_orbs,
                move_enemies,
                fire_fireballs,
                fire_orbs,
                spawn_enemies,
                animate_sprite,
                handle_invincibility,
                handle_damage_flash,
                handle_spell_collisions,
                handle_enemy_player_collisions,
                handle_death,
                collect_gems,
                update_hud,
                update_weapon_hud,
                toggle_map,
                update_map,
                update_terrain,
            )
                .into_configs()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            transition_to_levelup
                .into_configs()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::LevelUp), spawn_levelup_menu)
        .add_systems(
            Update,
            handle_skill_selection.run_if(in_state(GameState::LevelUp)),
        )
        .add_systems(OnEnter(GameState::GameOver), spawn_gameover_menu)
        .add_systems(Update, handle_restart.run_if(in_state(GameState::GameOver)))
        .run();
}
