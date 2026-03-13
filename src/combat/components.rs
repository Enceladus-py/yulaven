use bevy::{
    math::Vec2,
    prelude::{Component, Transform},
    sprite::Sprite,
    time::Timer,
};

#[derive(Component, Default)]
#[require(Sprite, Transform)]
pub struct Fireball {
    pub progress: f32,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct FireballAnimation {
    pub timer: Timer,
    pub last_frame: usize,
    pub first_frame: usize,
}

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Orb {
    pub direction: Vec2,
}

#[derive(Component, Default)]
pub struct Spell {
    pub damage: f32,
}

#[derive(Component)]
pub struct Invincible(pub Timer);

#[derive(Component)]
pub struct Knockback {
    pub velocity: Vec2,
    pub timer: Timer,
}
