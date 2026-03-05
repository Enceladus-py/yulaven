use bevy::{
    math::Vec2,
    prelude::{Component, Transform},
    sprite::Sprite,
};

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Orb {
    pub direction: Vec2,
}
