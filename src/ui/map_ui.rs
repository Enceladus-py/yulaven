use crate::player::components::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct MinimapUi;

#[derive(Component)]
pub struct LargeMapUi;

#[derive(Component)]
pub struct MinimapPlayerBlip;

#[derive(Component)]
pub struct LargeMapPlayerBlip;

#[derive(Component)]
pub struct UiTerrainTile {
    pub offset: IVec2,
}

pub fn spawn_minimap_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(16.0),
                top: Val::Px(16.0),
                ..Default::default()
            },
            MinimapUi,
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(150.0),
                    border: UiRect::all(Val::Px(4.0)),
                    overflow: Overflow::clip(),
                    ..Default::default()
                },
                BorderColor::all(Color::srgb(0.2, 0.2, 0.2)),
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            ))
            .with_children(|map| {
                // Spawn 4x4 grid of terrain colors, absolutely positioned
                for y in -1..=2 {
                    for x in -1..=2 {
                        map.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                width: Val::Percent(33.333),
                                height: Val::Percent(33.333),
                                ..Default::default()
                            },
                            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                            UiTerrainTile {
                                offset: IVec2::new(x, -y),
                            },
                        ));
                    }
                }

                map.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(6.0),
                        height: Val::Px(6.0),
                        left: Val::Percent(50.0),
                        top: Val::Percent(50.0),
                        margin: UiRect {
                            left: Val::Px(-3.0),
                            top: Val::Px(-3.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
                    MinimapPlayerBlip,
                ));
            });
        });
}

pub fn spawn_large_map(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                margin: UiRect {
                    left: Val::Px(-300.0),
                    top: Val::Px(-300.0),
                    ..Default::default()
                },
                width: Val::Px(600.0),
                height: Val::Px(600.0),
                border: UiRect::all(Val::Px(8.0)),
                overflow: Overflow::clip(),
                display: Display::None,
                ..Default::default()
            },
            BorderColor::all(Color::srgb(0.5, 0.4, 0.2)),
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            LargeMapUi,
        ))
        .with_children(|map| {
            map.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            })
            .with_children(|inner_map| {
                // Spawn 4x4 grid of terrain colors, absolutely positioned
                for y in -1..=2 {
                    for x in -1..=2 {
                        inner_map.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                width: Val::Percent(33.333),
                                height: Val::Percent(33.333),
                                ..Default::default()
                            },
                            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                            UiTerrainTile {
                                offset: IVec2::new(x, -y),
                            },
                        ));
                    }
                }

                inner_map.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(12.0),
                        height: Val::Px(12.0),
                        left: Val::Percent(50.0),
                        top: Val::Percent(50.0),
                        margin: UiRect {
                            left: Val::Px(-6.0),
                            top: Val::Px(-6.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
                    LargeMapPlayerBlip,
                ));
            });
        });
}

pub fn toggle_map(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Node, With<LargeMapUi>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM)
        && let Ok(mut node) = query.single_mut()
    {
        node.display = if node.display == Display::None {
            Display::Flex
        } else {
            Display::None
        };
    }
}

#[allow(
    clippy::type_complexity,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
pub fn update_map(
    player_query: Query<&Transform, With<Player>>,
    mut tile_query: Query<(&UiTerrainTile, &mut Node, &mut BackgroundColor)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let px = player_transform.translation.x;
    let py = player_transform.translation.y;

    let tile_size = 4096.0;

    let base_x = (px / tile_size).round();
    let base_y = (py / tile_size).round();

    let pct_offset_x = (px - base_x * tile_size) / tile_size * 100.0;
    let pct_offset_y = -(py - base_y * tile_size) / tile_size * 100.0;

    for (tile, mut node, mut bg) in &mut tile_query {
        let logical_x = base_x as i32 + tile.offset.x;
        let logical_y = base_y as i32 + tile.offset.y;

        let base_seed =
            (logical_x.wrapping_mul(73_856_093_i32)) ^ (logical_y.wrapping_mul(19_349_663_i32));

        let rng_terrain = crate::map::systems::pcg_hash(base_seed as u32);

        if rng_terrain < 0.3 {
            *bg = BackgroundColor(Color::srgb(0.2, 0.6, 0.2));
        } else if rng_terrain < 0.5 {
            *bg = BackgroundColor(Color::srgb(0.1, 0.4, 0.1));
        } else if rng_terrain < 0.7 {
            *bg = BackgroundColor(Color::srgb(0.6, 0.4, 0.2));
        } else if rng_terrain < 0.85 {
            *bg = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
        } else {
            *bg = BackgroundColor(Color::srgb(0.8, 0.7, 0.3));
        }

        let view_center = 50.0;
        let block_radius = 33.333 / 2.0;

        let start_left = view_center - block_radius + (tile.offset.x as f32 * 33.333);
        let start_top = view_center - block_radius + (-tile.offset.y as f32 * 33.333);

        node.left = Val::Percent(start_left - (pct_offset_x * 0.33333));
        node.top = Val::Percent(start_top - (pct_offset_y * 0.33333));
    }
}
