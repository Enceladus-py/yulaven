use bevy::prelude::Component;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct DespawnNextFrame;
