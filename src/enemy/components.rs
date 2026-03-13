use bevy::{
    prelude::{Component, Transform},
    sprite::Sprite,
};

#[derive(Component, Default)]
#[require(Sprite, Transform)]
pub struct Enemy {
    pub active: bool,
}

#[derive(Component)]
pub struct DamageFlash(pub bevy::time::Timer);
