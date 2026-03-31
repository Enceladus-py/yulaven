use bevy::prelude::*;

use super::components::CombatAssets;

/// System for pre-loading projectile textures and atlas layout.
pub fn setup_combat_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image = asset_server.load("HumansProjectiles.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 5, None, None);
    let layout_handle = texture_atlases.add(layout);
    commands.insert_resource(CombatAssets {
        projectile_image: image,
        atlas_layout: layout_handle,
    });
}

/// System for animating spell projectiles (fireballs, orbs).
pub fn animate_spell(
    time: Res<Time>,
    mut fb_query: Query<(&mut Sprite, &mut super::components::FireballAnimation)>,
) {
    for (mut sprite, mut animation) in &mut fb_query {
        animation.timer.tick(time.delta());
        if !animation.timer.just_finished() {
            continue;
        }
        let Some(ref mut atlas) = sprite.texture_atlas else {
            continue;
        };
        atlas.index = if atlas.index >= animation.last_frame || atlas.index < animation.first_frame
        {
            animation.first_frame
        } else {
            atlas.index + 1
        };
    }
}
