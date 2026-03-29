use bevy::{prelude::*, time::{Timer, TimerMode}};

/// The hero the player has selected. Set on the character-select screen,
/// read during `OnEnter(Playing)` to configure the player entity.
#[derive(Resource, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectedCharacter {
    /// The Mage — fires homing fireballs via orb charges.
    /// Active: Blink — teleports toward joystick/cursor direction.
    #[default]
    Mage,
    /// The Archer — rapid multi-shot orbs with wider spread.
    /// Active: Arrow Rain — 8 projectiles fired in a ring.
    Archer,
    /// The Warlock — slow but powerful; life-drain passive.
    /// Active: Void Nova — same as Nova but heals the player.
    Warlock,
}

impl SelectedCharacter {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Mage    => "Mage",
            Self::Archer  => "Archer",
            Self::Warlock => "Warlock",
        }
    }

    pub fn passive_description(self) -> &'static str {
        match self {
            Self::Mage    => "Charges up orbs → launches homing Fireball",
            Self::Archer  => "Rapid multi-shot orbs with wide spread",
            Self::Warlock => "Life-drain: heals 1 HP on every kill",
        }
    }

    pub fn active_description(self) -> &'static str {
        match self {
            Self::Mage    => "Blink — Teleport 320 px forward (5 s cooldown)",
            Self::Archer  => "Arrow Rain — Fire 8 orbs in all directions (10 s)",
            Self::Warlock => "Void Nova — Massive AoE + 8 HP heal (10 s)",
        }
    }

    pub fn accent_color(self) -> Color {
        match self {
            Self::Mage    => Color::srgb(0.3, 0.6, 1.0),
            Self::Archer  => Color::srgb(0.2, 0.9, 0.4),
            Self::Warlock => Color::srgb(0.7, 0.2, 1.0),
        }
    }

    pub fn card_color(self) -> Color {
        match self {
            Self::Mage    => Color::srgb(0.1, 0.15, 0.35),
            Self::Archer  => Color::srgb(0.08, 0.22, 0.1),
            Self::Warlock => Color::srgb(0.18, 0.05, 0.28),
        }
    }

    /// Cooldown in seconds for the active ability.
    pub fn active_cooldown_secs(self) -> f32 {
        match self {
            Self::Mage    => 5.0,
            Self::Archer | Self::Warlock => 10.0,
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
        let mut t = Timer::from_seconds(character.active_cooldown_secs(), TimerMode::Once);
        // Start ready
        t.tick(std::time::Duration::from_secs_f32(character.active_cooldown_secs()));
        Self {
            cooldown: t,
            kind: character,
        }
    }
}
