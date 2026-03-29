use bevy::prelude::*;

pub mod components;
pub mod setup;
pub mod state;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<state::GameState>()
            // Camera + terrain always present from the start
            .add_systems(Startup, setup::setup)
            // Player and terrain spawn only when we leave CharacterSelect
            .add_systems(
                OnExit(state::GameState::CharacterSelect),
                (setup::spawn_player, setup::spawn_terrain),
            )
            .add_systems(Last, cleanup_despawns);
    }
}

fn cleanup_despawns(mut commands: Commands, q: Query<Entity, With<components::DespawnNextFrame>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
