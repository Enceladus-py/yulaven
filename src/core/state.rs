use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    /// Initial screen — Yulaven title & Play button.
    #[default]
    MainMenu,
    /// Second screen — player picks their hero.
    CharacterSelect,
    /// Detailed view of a character before selection.
    CharacterDetail,
    Playing,
    LevelUp,
    Paused,
    GameOver,
}
