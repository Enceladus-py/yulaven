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
    pub pine_tree: Handle<Image>,
    pub stone_rocks: Vec<Handle<Image>>,
    pub ruined_pillars: Vec<Handle<Image>>,
}
