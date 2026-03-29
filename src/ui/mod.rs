use bevy::prelude::*;

pub mod character_select;
pub mod game_over;
pub mod hud;
pub mod joystick;
pub mod level_up;
pub mod main_menu;
pub mod map_ui;

// Re-export common UI components/markers if needed widely
pub use game_over::*;
pub use hud::*;
pub use joystick::JoystickInput;
pub use level_up::*;
pub use map_ui::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<joystick::JoystickInput>()
            .init_resource::<joystick::JoystickFinger>()
            .add_plugins(UiMaterialPlugin::<hud::CircularCooldownMaterial>::default())
            // ── Main Menu ────────────────────────────────────────────────────
            .add_systems(
                OnEnter(crate::GameState::MainMenu),
                main_menu::spawn_main_menu,
            )
            .add_systems(
                Update,
                (
                    main_menu::handle_main_menu,
                    main_menu::highlight_play_button,
                )
                    .run_if(in_state(crate::GameState::MainMenu)),
            )
            // ── Character select ─────────────────────────────────────────────
            .add_systems(
                OnEnter(crate::GameState::CharacterSelect),
                character_select::spawn_character_select,
            )
            .add_systems(
                Update,
                (
                    character_select::handle_character_select,
                    character_select::highlight_character_card,
                )
                    .run_if(in_state(crate::GameState::CharacterSelect)),
            )
            // ── In-game HUD (spawned when exiting CharacterSelect) ───────────────
            .add_systems(
                OnExit(crate::GameState::CharacterSelect),
                (
                    hud::build_mobile_hud,
                    map_ui::spawn_minimap_hud,
                    map_ui::spawn_large_map,
                ),
            )
            .add_systems(
                Update,
                (
                    hud::update_mobile_hud,
                    map_ui::toggle_map,
                    map_ui::update_map,
                    map_ui::update_minimap_enemy_blips,
                )
                    .run_if(in_state(crate::GameState::Playing)),
            )
            .add_systems(
                Update,
                level_up::transition_to_levelup.run_if(in_state(crate::GameState::Playing)),
            );

        #[cfg(any(target_os = "android", target_os = "ios"))]
        app.add_systems(
            OnExit(crate::GameState::CharacterSelect),
            joystick::spawn_joystick,
        )
        .add_systems(
            Update,
            joystick::update_joystick.run_if(in_state(crate::GameState::Playing)),
        );

        app.add_systems(
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
