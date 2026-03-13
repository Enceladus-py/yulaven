use bevy::prelude::*;

pub mod components;
pub mod setup;
pub mod state;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<state::GameState>()
            .add_systems(Startup, setup::setup);
    }
}
