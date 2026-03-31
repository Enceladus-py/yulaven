use bevy::prelude::*;

use super::definition::CharacterDefinition;

/// The hero the player has selected. Set on the character-select screen,
/// read during `OnEnter(Playing)` to configure the player entity.
#[derive(Resource, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectedCharacter {
    #[default]
    Mage,
    Archer,
    Warlock,
}

impl SelectedCharacter {
    pub fn definition(self) -> CharacterDefinition {
        CharacterDefinition::from_selected(self)
    }

    pub fn display_name(self) -> &'static str {
        self.definition().display_name
    }

    pub fn passive_description(self) -> &'static str {
        self.definition().passive_description
    }

    pub fn active_description(self) -> &'static str {
        self.definition().active_description
    }

    pub fn accent_color(self) -> Color {
        self.definition().accent_color
    }

    pub fn card_color(self) -> Color {
        self.definition().card_color
    }

    pub fn active_cooldown_secs(self) -> f32 {
        self.definition().active_cooldown_secs
    }

    /// Returns a simple health indicator string
    pub fn health_indicator(self) -> &'static str {
        match self {
            SelectedCharacter::Warlock => "♥♥♥",
            SelectedCharacter::Mage => "♥♥",
            SelectedCharacter::Archer => "♥",
        }
    }
}

/// Component added to the player entity; tracks the active-ability cooldown.
#[derive(Component)]
pub struct ActiveAbility {
    pub cooldown: Timer,
    pub kind: SelectedCharacter,
}

impl ActiveAbility {
    pub fn new(character: SelectedCharacter) -> Self {
        let def = character.definition();
        let mut t = Timer::from_seconds(def.active_cooldown_secs, TimerMode::Once);
        t.tick(std::time::Duration::from_secs_f32(def.active_cooldown_secs));
        Self {
            cooldown: t,
            kind: character,
        }
    }
}
