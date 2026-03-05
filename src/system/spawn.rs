use bevy::prelude::*;

use crate::component::{
    enemy::Enemy,
    fireball::{Fireball, FireballAnimation},
    orb::Orb,
    player::{Player, PlayerAnimation},
    spell::Spell,
};

const ORB_CHARGES_NEEDED: u8 = 5;

pub fn fire_fireballs(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &Sprite, &PlayerAnimation, &mut Player)>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if let Ok((player_transform, sprite, _anim, mut player)) = player_query.get_single_mut() {
        player.fireball_timer.tick(time.delta());

        // Only fire when cooldown is done AND enough orb charges accumulated
        if !player.fireball_timer.just_finished() || player.orb_charges < ORB_CHARGES_NEEDED {
            return;
        }
        player.orb_charges = 0;

        // Aim at nearest enemy
        let Some(enemy_transform) = nearest_enemy(player_transform.translation, &enemy_query)
        else {
            return;
        };

        let texture_handle = asset_server.load("HumansProjectiles.png");
        let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 5, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let fb_offset_x = if sprite.flip_x { -20.0 } else { 20.0 };
        let direction = (enemy_transform.translation - player_transform.translation).truncate();
        let fireball_direction = direction.normalize_or_zero();
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
            Spell { damage: 25.0 },
        ));
    }
}

pub fn fire_orbs(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &Sprite, &PlayerAnimation, &mut Player)>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if let Ok((player_transform, sprite, _anim, mut player)) = player_query.get_single_mut() {
        player.orb_timer.tick(time.delta());
        if !player.orb_timer.just_finished() {
            return;
        }

        // Increment charge toward fireball
        player.orb_charges = player.orb_charges.saturating_add(1);

        let texture_handle = asset_server.load("HumansProjectiles.png");
        let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 5, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let offset_x = if sprite.flip_x { -20.0 } else { 20.0 };

        // Aim at nearest enemy; fall back to facing direction
        let initial_direction = nearest_enemy(player_transform.translation, &enemy_query).map_or(
            player.facing_direction,
            |et| {
                (et.translation - player_transform.translation)
                    .truncate()
                    .normalize_or_zero()
            },
        );

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
            Orb {
                direction: initial_direction,
            },
            Spell { damage: 15.0 },
        ));
    }
}

/// Returns the transform of the enemy closest to `origin`, or `None` if no enemies exist.
fn nearest_enemy<'a>(
    origin: Vec3,
    enemy_query: &'a Query<&Transform, With<Enemy>>,
) -> Option<&'a Transform> {
    enemy_query.iter().min_by(|a, b| {
        let da = a.translation.distance_squared(origin);
        let db = b.translation.distance_squared(origin);
        da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
    })
}
