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

#[derive(Resource)]
pub struct StructureAssets {
    pub pine_tree: Handle<Image>,
    pub stone_rocks: Vec<Handle<Image>>,
    pub ruined_pillars: Vec<Handle<Image>>,
}
