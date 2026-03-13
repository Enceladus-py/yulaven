use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (systems::update_terrain, systems::collect_gems)
                .run_if(in_state(crate::GameState::Playing)),
        );
    }
}
