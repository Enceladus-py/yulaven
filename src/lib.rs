#![deny(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

#[cfg(target_os = "android")]
use bevy::prelude::bevy_main;
use bevy::prelude::*;

mod combat;
mod constant;
mod core;
mod enemy;
mod map;
mod player;
pub mod ui;

pub use core::state::GameState;

#[cfg(target_os = "android")]
#[bevy_main]
fn main() {
    run_game();
}

pub fn run_game() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Arcane Survivors".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(core::CorePlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(combat::CombatPlugin)
        .add_plugins(map::MapPlugin)
        .add_plugins(ui::UiPlugin)
        .insert_resource(ClearColor(Color::srgb(0.5, 0.1, 0.1))) // Bright red to see clearly
        .add_message::<player::components::LevelUpEvent>()
        .run();
}
