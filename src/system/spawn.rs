use bevy::prelude::*;

use crate::component::{
    enemy::Enemy,
    fireball::{Fireball, FireballAnimation},
    orb::Orb,
    player::{Player, PlayerAnimation},
    spell::Spell,
};

pub fn fire_fireballs(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &Sprite, &PlayerAnimation, &mut Player)>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if let (Ok((player_transform, sprite, _player_animation, mut player)), Ok(enemy_transform)) =
        (player_query.get_single_mut(), enemy_query.get_single())
    {
        player.fireball_timer.tick(time.delta());
        if player.fireball_timer.just_finished() {
            let texture_handle = asset_server.load("HumansProjectiles.png");
            let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 5, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            let fb_offset_x = if sprite.flip_x { -20.0 } else { 20.0 };

            // Calculate direction from player to nearest enemy
            let direction = (enemy_transform.translation - player_transform.translation).truncate();
            let fireball_direction = direction.normalize_or_zero();

            // Calculate rotation angle to face the target
            let fireball_rotation =
                Quat::from_rotation_z(fireball_direction.y.atan2(fireball_direction.x));

            commands.spawn((
                Sprite {
                    image: texture_handle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_handle,
                        index: 5,
                    }),
                    ..Default::default()
                },
                Transform {
                    translation: player_transform.translation
                        + Vec3 {
                            x: fb_offset_x,
                            y: 0.0,
                            z: 0.0,
                        },
                    rotation: fireball_rotation,
                    scale: Vec3::splat(4.0),
                },
                Fireball {
                    progress: 0.0,
                    direction: fireball_direction,
                },
                FireballAnimation {
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    first_frame: 5,
                    last_frame: 6,
                },
                Spell { damage: 10.0 },
            ));
        }
    }
}

pub fn fire_orbs(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &Sprite, &PlayerAnimation, &mut Player)>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    if let Ok((player_transform, sprite, _player_animation, mut player)) =
        player_query.get_single_mut()
    {
        player.orb_timer.tick(time.delta());
        if player.orb_timer.just_finished() {
            let texture_handle = asset_server.load("HumansProjectiles.png");
            let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 5, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            let offset_x = if sprite.flip_x { -20.0 } else { 20.0 };

            commands.spawn((
                Sprite {
                    image: texture_handle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_handle,
                        index: 10,
                    }),
                    flip_x: sprite.flip_x,
                    ..Default::default()
                },
                Transform {
                    translation: player_transform.translation
                        + Vec3 {
                            x: offset_x,
                            y: 0.0,
                            z: 0.0,
                        },
                    scale: Vec3::splat(4.0),
                    ..Default::default()
                },
                Orb,
                Spell { damage: 15.0 },
            ));
        }
    }
}
