use bevy::prelude::*;

use super::components::{Health, MainCamera};
use crate::enemy::components::Enemy;
use crate::map::components::{Collider, Structure, StructureAssets, TerrainTile};
use crate::player::components::{Player, PlayerAnimation, PlayerStats};

#[allow(clippy::cast_precision_loss, clippy::too_many_lines)]
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((Camera2d, MainCamera));

    let pine_tree: Handle<Image> = asset_server.load("structures/pine_tree.png");

    let mut stone_rocks = Vec::new();
    for i in 0..16 {
        stone_rocks.push(asset_server.load(format!("structures/stone_rock_{i}.png")));
    }

    let mut ruined_pillars = Vec::new();
    for i in 0..4 {
        ruined_pillars.push(asset_server.load(format!("structures/ruined_pillar_{i}.png")));
    }

    let grass_terrain: Handle<Image> = asset_server.load_with_settings(
        "textures/grass_terrain.png",
        |settings: &mut bevy::image::ImageLoaderSettings| {
            settings.sampler =
                bevy::image::ImageSampler::Descriptor(bevy::image::ImageSamplerDescriptor {
                    address_mode_u: bevy::image::ImageAddressMode::Repeat,
                    address_mode_v: bevy::image::ImageAddressMode::Repeat,
                    ..Default::default()
                });
        },
    );
    let dirt_terrain: Handle<Image> = asset_server.load_with_settings(
        "textures/dirt_terrain.png",
        |settings: &mut bevy::image::ImageLoaderSettings| {
            settings.sampler =
                bevy::image::ImageSampler::Descriptor(bevy::image::ImageSamplerDescriptor {
                    address_mode_u: bevy::image::ImageAddressMode::Repeat,
                    address_mode_v: bevy::image::ImageAddressMode::Repeat,
                    ..Default::default()
                });
        },
    );

    let stone_terrain: Handle<Image> = asset_server.load_with_settings(
        "textures/stone_terrain.png",
        |settings: &mut bevy::image::ImageLoaderSettings| {
            settings.sampler =
                bevy::image::ImageSampler::Descriptor(bevy::image::ImageSamplerDescriptor {
                    address_mode_u: bevy::image::ImageAddressMode::Repeat,
                    address_mode_v: bevy::image::ImageAddressMode::Repeat,
                    ..Default::default()
                });
        },
    );
    let sand_terrain: Handle<Image> = asset_server.load_with_settings(
        "textures/sand_terrain.png",
        |settings: &mut bevy::image::ImageLoaderSettings| {
            settings.sampler =
                bevy::image::ImageSampler::Descriptor(bevy::image::ImageSamplerDescriptor {
                    address_mode_u: bevy::image::ImageAddressMode::Repeat,
                    address_mode_v: bevy::image::ImageAddressMode::Repeat,
                    ..Default::default()
                });
        },
    );
    let dark_grass_terrain: Handle<Image> = asset_server.load_with_settings(
        "textures/dark_grass_terrain.png",
        |settings: &mut bevy::image::ImageLoaderSettings| {
            settings.sampler =
                bevy::image::ImageSampler::Descriptor(bevy::image::ImageSamplerDescriptor {
                    address_mode_u: bevy::image::ImageAddressMode::Repeat,
                    address_mode_v: bevy::image::ImageAddressMode::Repeat,
                    ..Default::default()
                });
        },
    );

    commands.insert_resource(StructureAssets {
        grass_terrain: grass_terrain.clone(),
        dirt_terrain,
        stone_terrain,
        sand_terrain,
        dark_grass_terrain,
        pine_tree: pine_tree.clone(),
        stone_rocks,
        ruined_pillars,
    });

    let terrain_handle = grass_terrain;
    for x in -1..=1 {
        for y in -1..=1 {
            commands
                .spawn((
                    Sprite {
                        color: Color::srgb(0.4, 0.4, 0.4),
                        image: terrain_handle.clone(),
                        custom_size: Some(Vec2::new(4096.0, 4096.0)),
                        image_mode: SpriteImageMode::Tiled {
                            tile_x: true,
                            tile_y: true,
                            stretch_value: 1.0,
                        },
                        ..Default::default()
                    },
                    Transform::from_xyz(x as f32 * 4096.0, y as f32 * 4096.0, -10.0),
                    TerrainTile {
                        offset: IVec2::new(x, y),
                        logical_coord: IVec2::new(-999, -999),
                    },
                ))
                .with_children(|parent| {
                    for i in 0..20 {
                        parent.spawn((
                            Sprite {
                                color: Color::srgb(0.4, 0.4, 0.4),
                                image: pine_tree.clone(),
                                ..Default::default()
                            },
                            Transform::from_xyz(0.0, 0.0, 1.0),
                            Structure { local_index: i },
                            Collider { radius: 100.0 },
                        ));
                    }
                });
        }
    }

    let texture_handle = asset_server.load("outline/MiniMage.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(32), 12, 12, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Sprite {
            image: texture_handle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_handle,
                index: 0,
            }),
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::splat(4.0),
            ..Default::default()
        },
        Player {
            facing_direction: Vec2::new(1.0, 0.0),
            ..Default::default()
        },
        Health(100.0),
        PlayerStats::default(),
        PlayerAnimation {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            first_frame: 0,
            last_frame: 3,
            attack_timer: Timer::from_seconds(0.0, TimerMode::Once),
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
        Enemy { active: false },
    ));
}
