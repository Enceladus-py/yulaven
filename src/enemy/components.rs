use bevy::{
    prelude::{Color, Component, Transform},
    sprite::Sprite,
    time::Timer,
};

/// Which variant of enemy this entity represents.
/// Drives stats, color, and behaviour.
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum EnemyKind {
    /// Slow melee attacker — the classic enemy (dark red)
    Grunt,
    /// Fast, low-HP — rushes the player (orange)
    Runner,
    /// Slow, very tanky — resists knockback (purple)
    Brute,
    /// Medium speed, phases through map structures (cyan/teal)
    Specter,
    /// Small and weak, drops extra XP (bright green)
    Goblin,
}

impl EnemyKind {
    /// Returns true for the Specter variant so movement can skip collision.
    pub fn ignores_structures(self) -> bool {
        matches!(self, Self::Specter)
    }

    pub fn base_health(self) -> f32 {
        match self {
            Self::Grunt => 30.0,
            Self::Runner => 14.0,
            Self::Brute => 90.0,
            Self::Specter => 25.0,
            Self::Goblin => 10.0,
        }
    }

    pub fn base_speed(self) -> f32 {
        match self {
            Self::Grunt => 50.0,
            Self::Runner => 100.0,
            Self::Brute => 30.0,
            Self::Specter => 65.0,
            Self::Goblin => 70.0,
        }
    }

    pub fn contact_damage(self) -> f32 {
        match self {
            Self::Grunt | Self::Runner => 1.0,
            Self::Brute => 3.0,
            Self::Specter => 2.0,
            Self::Goblin => 0.5,
        }
    }

    pub fn xp_drop(self) -> f32 {
        match self {
            Self::Grunt => 10.0,
            Self::Runner => 8.0,
            Self::Brute => 25.0,
            Self::Specter => 15.0,
            Self::Goblin => 18.0, // goblins are XP pinatas
        }
    }

    pub fn color(self) -> Color {
        match self {
            Self::Grunt => Color::srgb(0.6, 0.2, 0.2),
            Self::Runner => Color::srgb(0.9, 0.45, 0.05),
            Self::Brute => Color::srgb(0.45, 0.1, 0.7),
            Self::Specter => Color::srgba(0.1, 0.75, 0.85, 0.65),
            Self::Goblin => Color::srgb(0.2, 0.75, 0.2),
        }
    }

    /// Sprite size in world units (width, height)
    pub fn size(self) -> bevy::math::Vec2 {
        use bevy::math::Vec2;
        match self {
            Self::Grunt => Vec2::new(50.0, 50.0),
            Self::Runner => Vec2::new(38.0, 38.0),
            Self::Brute => Vec2::new(70.0, 70.0),
            Self::Specter => Vec2::new(50.0, 60.0),
            Self::Goblin => Vec2::new(30.0, 30.0),
        }
    }
}

/// Per-instance stats derived from `EnemyKind` at spawn time.
#[derive(Component)]
pub struct EnemyStats {
    pub speed: f32,
    pub contact_damage: f32,
    pub xp_drop: f32,
}

#[derive(Component, Default)]
#[require(Sprite, Transform)]
pub struct Enemy {
    pub active: bool,
}

#[derive(Component)]
pub struct DamageFlash(pub Timer);
