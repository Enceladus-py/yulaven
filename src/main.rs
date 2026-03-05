#![deny(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

use bevy::prelude::*;
use system::{
    animation::animate_sprite,
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
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player,
                move_fireballs,
                move_orbs,
                fire_fireballs,
                fire_orbs,
                animate_sprite,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}
