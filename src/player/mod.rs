use bevy::prelude::*;

pub mod active_ability;
pub mod character;
pub mod components;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // HUD spawn handled in ui plugin

        app.init_resource::<character::SelectedCharacter>()
            .add_systems(
                Update,
                (
                    systems::move_player,
                    systems::handle_teleportation,
                    systems::animate_dash_trail,
                    systems::animate_player,
                    active_ability::trigger_active_ability,
                )
                    .run_if(in_state(crate::GameState::Playing)),
            );
    }
}
