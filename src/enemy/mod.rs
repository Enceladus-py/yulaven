use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<systems::EnemySpawnTimer>()
            .init_resource::<systems::GameTimer>()
            .add_systems(
                Update,
                (systems::spawn_enemies, systems::move_enemies)
                    .run_if(in_state(crate::GameState::Playing)),
            );
    }
}
