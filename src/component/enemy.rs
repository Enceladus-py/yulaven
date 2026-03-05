use bevy::{
    prelude::{Component, Transform},
    sprite::Sprite,
};

#[derive(Component, Default)]
#[require(Sprite, Transform)]
pub struct Enemy {
    pub active: bool,
}
