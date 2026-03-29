use bevy::{
    math::Vec2,
    prelude::{Component, Message},
    sprite::Sprite,
    time::{Timer, TimerMode},
};

#[derive(Component)]
#[require(Sprite)]
pub struct Player {
    pub facing_direction: Vec2,
    pub fireball_timer: Timer,
    pub orb_timer: Timer,
    pub orb_charges: u8,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            facing_direction: Vec2::X,
            fireball_timer: Timer::from_seconds(0.4, TimerMode::Repeating),
            orb_timer: Timer::from_seconds(1.5, TimerMode::Repeating),
            orb_charges: 0,
        }
    }
}

#[derive(Component)]
pub struct PlayerAnimation {
    pub timer: Timer,
    pub last_frame: usize,
    pub first_frame: usize,
    pub attack_timer: Timer,
}

#[derive(Component)]
pub struct PlayerStats {
    pub level: u32,
    pub current_xp: f32,
    pub required_xp: f32,
    pub magnet_radius: f32,
    /// World-unit radius in which the player will target enemies (must be on screen)
    pub attack_range: f32,
    /// Multiplier applied to all outgoing spell damage (1.0 = base)
    pub damage_multiplier: f32,
    /// Multiplier applied to player movement speed (1.0 = base)
    pub speed_multiplier: f32,
    /// Current maximum HP ceiling (drives health bar display)
    pub max_health: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            level: 1,
            current_xp: 0.0,
            required_xp: 10.0,
            magnet_radius: 150.0,
            attack_range: 500.0,
            damage_multiplier: 1.0,
            speed_multiplier: 1.0,
            max_health: 100.0,
        }
    }
}

// Event to signal level up
#[derive(Message)]
pub struct LevelUpEvent;
