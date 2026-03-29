use bevy::prelude::*;

pub mod active_ability;
pub mod character;
pub mod components;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<character::SelectedCharacter>()
            .add_systems(
                OnExit(crate::GameState::CharacterSelect),
                active_ability::spawn_active_ability_hud,
            )
            .add_systems(
                Update,
                (
                    systems::move_player,
                    systems::animate_player,
                    active_ability::trigger_active_ability,
                    active_ability::update_active_ability_hud,
                )
                    .run_if(in_state(crate::GameState::Playing)),
            );
    }
}

