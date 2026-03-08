use bevy::prelude::*;

use crate::component::{
    map::{Structure, TerrainTile},
    player::Player,
};

const TILE_SIZE: f32 = 4096.0;

// Simple deterministic pseudo-random hash function based on world coords
// Returns float between 0.0 and 1.0
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
    structure_assets: Res<crate::component::map::StructureAssets>,
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

        // If the tile just spawned into this logical position, refresh its structures
        if needs_refresh {
            let base_seed = (terrain.logical_coord.x.wrapping_mul(73_856_093_i32))
                ^ (terrain.logical_coord.y.wrapping_mul(19_349_663_i32));

            // Randomly pick terrain tile
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

                    // Random position within the tile
                    str_transform.translation.x = (rng_x - 0.5) * TILE_SIZE;
                    str_transform.translation.y = (rng_y - 0.5) * TILE_SIZE;
                    str_transform.translation.z = 1.0 - (str_transform.translation.y / TILE_SIZE);

                    // Assign sprite based on random value
                    if rng_type < 0.6 {
                        sprite.image = structure_assets.pine_tree.clone();
                    } else if rng_type < 0.9 {
                        let idx = (pcg_hash(seed.wrapping_add(3)) * 16.0) as usize % 16;
                        sprite.image = structure_assets.stone_rocks[idx].clone();
                    } else {
                        let idx = (pcg_hash(seed.wrapping_add(3)) * 4.0) as usize % 4;
                        sprite.image = structure_assets.ruined_pillars[idx].clone();
                    }
                }
            }
        }
    }
}
