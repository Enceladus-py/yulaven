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
pub struct Invincible(pub Timer);

#[derive(Component)]
pub struct Knockback {
    pub velocity: Vec2,
    pub timer: Timer,
}
