use bevy::prelude::*;

use crate::component::{
    camera::MainCamera,
    enemy::Enemy,
    experience::PlayerStats,
    health::Health,
    player::{Player, PlayerAnimation},
};

// Setup the game: spawn the camera and player entity
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Camera
    commands.spawn((Camera2d, MainCamera));

    let texture_handle = asset_server.load("outline/MiniMage.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(32), 12, 12, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Sprite {
            image: texture_handle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_handle,
                index: 0, // Start with the first frame
            }),
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::splat(4.0),
            ..Default::default()
        },
        Player {
            facing_direction: Vec2::new(1.0, 0.0), // facing right
            ..Default::default()
        },
        Health(100.0),
        PlayerStats::default(),
        PlayerAnimation {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            first_frame: 0,
            last_frame: 3,
        },
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.6, 0.6, 0.3),
            custom_size: Some(Vec2::new(100.0, 50.0)),
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(0.0, -50.0, 0.0),
            ..Default::default()
        },
        Enemy,
    ));
}
