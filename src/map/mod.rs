use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<components::MapSeed>()
            .add_systems(
                OnExit(crate::core::state::GameState::CharacterSelect),
                randomize_map_seed,
            )
            .add_systems(
                Update,
                (systems::update_terrain, systems::collect_gems)
                    .run_if(in_state(crate::core::state::GameState::Playing)),
            );
    }
}

fn randomize_map_seed(mut map_seed: ResMut<components::MapSeed>) {
    map_seed.0 = rand::random::<u32>();
}
