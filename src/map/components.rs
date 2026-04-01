use bevy::prelude::*;

#[derive(Component)]
pub struct TerrainTile {
    pub offset: IVec2,
    pub logical_coord: IVec2,
}

#[derive(Component)]
pub struct Structure {
    pub local_index: usize,
}

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}

#[derive(Resource)]
pub struct StructureAssets {
    pub grass_terrain: Handle<Image>,
    pub dirt_terrain: Handle<Image>,
    pub stone_terrain: Handle<Image>,
    pub sand_terrain: Handle<Image>,
    pub dark_grass_terrain: Handle<Image>,
    // Spritesheets
    pub trees_sheet: Handle<Image>,
    pub trees_layout: Handle<TextureAtlasLayout>,
    pub stones_sheet: Handle<Image>,
    pub stones_layout: Handle<TextureAtlasLayout>,
    pub pillars_sheet: Handle<Image>,
    pub pillars_layout: Handle<TextureAtlasLayout>,
}
