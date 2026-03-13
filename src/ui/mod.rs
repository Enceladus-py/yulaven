use bevy::prelude::*;

pub mod game_over;
pub mod hud;
pub mod level_up;
pub mod map_ui;

// Re-export common UI components/markers if needed widely
pub use game_over::*;
pub use hud::*;
pub use level_up::*;
pub use map_ui::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                hud::spawn_hud,
                hud::spawn_weapon_hud,
                map_ui::spawn_minimap_hud,
                map_ui::spawn_large_map,
            ),
        )
        .add_systems(
            Update,
            (
                hud::update_hud,
                hud::update_weapon_hud,
                map_ui::toggle_map,
                map_ui::update_map,
            )
                .run_if(in_state(crate::GameState::Playing)),
        )
        .add_systems(
            Update,
            level_up::transition_to_levelup.run_if(in_state(crate::GameState::Playing)),
        )
        .add_systems(
            OnEnter(crate::GameState::LevelUp),
            level_up::spawn_levelup_menu,
        )
        .add_systems(
            Update,
            level_up::handle_skill_selection.run_if(in_state(crate::GameState::LevelUp)),
        )
        .add_systems(
            OnEnter(crate::GameState::GameOver),
            game_over::spawn_gameover_menu,
        )
        .add_systems(
            Update,
            game_over::handle_restart.run_if(in_state(crate::GameState::GameOver)),
        );
    }
}
