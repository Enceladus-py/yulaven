use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (systems::move_player, systems::animate_player)
                .run_if(in_state(crate::GameState::Playing)),
        );
    }
}
