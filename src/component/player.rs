use bevy::{
    math::Vec2,
    prelude::Component,
    sprite::Sprite,
    time::{Timer, TimerMode},
};

#[derive(Component)]
#[require(Sprite)]
pub struct Player {
    pub facing_direction: Vec2,
    pub fireball_timer: Timer,
    pub orb_timer: Timer,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            facing_direction: Vec2::X,
            fireball_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            orb_timer: Timer::from_seconds(2.5, TimerMode::Repeating),
        }
    }
}

pub enum PlayerAttackMode {
    Orb,
    Fireball,
    None,
}

#[derive(Component)]
pub struct PlayerAnimation {
    pub timer: Timer,
    pub last_frame: usize,
    pub first_frame: usize,
    pub attack_mode: PlayerAttackMode,
}
