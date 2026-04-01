use bevy::prelude::*;

use super::components::{Structure, TerrainTile};
use crate::combat::components::ExperienceGem;
use crate::core::components::DespawnNextFrame;
use crate::player::components::Player;
use crate::player::components::{LevelUpEvent, PlayerStats};

const TILE_SIZE: f32 = 4096.0;

#[allow(clippy::cast_precision_loss)]
pub fn pcg_hash(state: u32) -> f32 {
    let word = ((state >> ((state >> 28) + 4)) ^ state).wrapping_mul(277_803_737);
    let result = (word >> 22) ^ word;
    (result as f32) / (u32::MAX as f32)
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::type_complexity,
    clippy::cast_possible_wrap
)]
pub fn update_terrain(
    player_query: Query<&Transform, With<Player>>,
    mut terrain_query: Query<
        (&mut Transform, &mut TerrainTile, &mut Sprite, &Children),
        Without<Player>,
    >,
    mut structure_query: Query<
        (&mut Transform, &mut Sprite, &Structure),
        (Without<Player>, Without<TerrainTile>),
    >,
    structure_assets: Res<crate::map::components::StructureAssets>,
    _asset_server: Res<AssetServer>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let grass_terrain = structure_assets.grass_terrain.clone();
    let dirt_terrain = structure_assets.dirt_terrain.clone();
    let stone_terrain = structure_assets.stone_terrain.clone();
    let sand_terrain = structure_assets.sand_terrain.clone();
    let dark_grass_terrain = structure_assets.dark_grass_terrain.clone();

    let px = player_transform.translation.x;
    let py = player_transform.translation.y;

    let base_x = (px / TILE_SIZE).round();
    let base_y = (py / TILE_SIZE).round();

    for (mut transform, mut terrain, mut tile_sprite, children) in &mut terrain_query {
        let logical_x = base_x as i32 + terrain.offset.x;
        let logical_y = base_y as i32 + terrain.offset.y;

        let mut needs_refresh = false;
        if terrain.logical_coord.x != logical_x || terrain.logical_coord.y != logical_y {
            terrain.logical_coord = IVec2::new(logical_x, logical_y);
            needs_refresh = true;
        }

        let target_x = terrain.logical_coord.x as f32 * TILE_SIZE;
        let target_y = terrain.logical_coord.y as f32 * TILE_SIZE;

        transform.translation.x = target_x;
        transform.translation.y = target_y;

        if needs_refresh {
            let base_seed = (terrain.logical_coord.x.wrapping_mul(73_856_093_i32))
                ^ (terrain.logical_coord.y.wrapping_mul(19_349_663_i32));

            let rng_terrain = pcg_hash(base_seed as u32);
            if rng_terrain < 0.3 {
                tile_sprite.image = grass_terrain.clone();
            } else if rng_terrain < 0.5 {
                tile_sprite.image = dark_grass_terrain.clone();
            } else if rng_terrain < 0.7 {
                tile_sprite.image = dirt_terrain.clone();
            } else if rng_terrain < 0.85 {
                tile_sprite.image = stone_terrain.clone();
            } else {
                tile_sprite.image = sand_terrain.clone();
            }

            for &child in children {
                if let Ok((mut str_transform, mut sprite, structure)) =
                    structure_query.get_mut(child)
                {
                    let seed = (base_seed
                        ^ (structure.local_index as i32).wrapping_mul(83_492_791_i32))
                        as u32;

                    let rng_x = pcg_hash(seed);
                    let rng_y = pcg_hash(seed.wrapping_add(1));
                    let rng_type = pcg_hash(seed.wrapping_add(2));

                    str_transform.translation.x = (rng_x - 0.5) * TILE_SIZE;
                    str_transform.translation.y = (rng_y - 0.5) * TILE_SIZE;
                    str_transform.translation.z = 1.0 - (str_transform.translation.y / TILE_SIZE);

                    if rng_type < 0.6 {
                        sprite.image = structure_assets.trees_sheet.clone();
                        sprite.texture_atlas = Some(TextureAtlas {
                            layout: structure_assets.trees_layout.clone(),
                            index: (pcg_hash(seed.wrapping_add(3)) * 64.0) as usize % 64,
                        });
                    } else if rng_type < 0.9 {
                        sprite.image = structure_assets.stones_sheet.clone();
                        sprite.texture_atlas = Some(TextureAtlas {
                            layout: structure_assets.stones_layout.clone(),
                            index: (pcg_hash(seed.wrapping_add(3)) * 64.0) as usize % 64,
                        });
                    } else {
                        sprite.image = structure_assets.pillars_sheet.clone();
                        sprite.texture_atlas = Some(TextureAtlas {
                            layout: structure_assets.pillars_layout.clone(),
                            index: (pcg_hash(seed.wrapping_add(3)) * 64.0) as usize % 64,
                        });
                    }
                }
            }
        }
    }
}

pub fn collect_gems(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut PlayerStats), With<Player>>,
    mut gem_query: Query<(Entity, &mut Transform, &ExperienceGem), Without<Player>>,
    mut ev_level_up: MessageWriter<LevelUpEvent>,
    time: Res<Time>,
) {
    if let Ok((player_transform, mut player_stats)) = player_query.single_mut() {
        let collect_radius = 30.0;
        let magnet_radius = player_stats.magnet_radius;
        let pull_speed = 400.0;

        for (gem_entity, mut gem_transform, gem) in &mut gem_query {
            let distance = player_transform
                .translation
                .distance(gem_transform.translation);

            if distance < magnet_radius && distance > collect_radius {
                let direction =
                    (player_transform.translation - gem_transform.translation).normalize_or_zero();
                gem_transform.translation += direction * pull_speed * time.delta_secs();
            }

            if distance < collect_radius {
                player_stats.current_xp += gem.amount;
                commands.entity(gem_entity).insert(DespawnNextFrame);

                if player_stats.current_xp >= player_stats.required_xp {
                    player_stats.level += 1;
                    player_stats.current_xp -= player_stats.required_xp;
                    player_stats.required_xp *= 1.5;
                    player_stats.magnet_radius += 10.0;
                    ev_level_up.write(LevelUpEvent);
                    info!("Level Up! Now level {}", player_stats.level);
                }
            }
        }
    }
}
