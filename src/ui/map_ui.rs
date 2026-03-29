use crate::enemy::components::{Enemy, EnemyKind};
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

/// Tag for the pre-spawned enemy blip pool nodes inside the minimap.
#[derive(Component)]
pub struct MinimapEnemyBlip {
    pub index: usize,
}

/// How many enemy blips the minimap pool contains.
pub const MAX_MINIMAP_BLIPS: usize = 30;

/// World-space radius the minimap covers (enemies beyond this are clamped to edge).
pub const MINIMAP_WORLD_RADIUS: f32 = 1200.0;

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
                    border_radius: BorderRadius::all(Val::Percent(50.0)),
                    ..Default::default()
                },
                BorderColor::all(Color::srgb(0.3, 0.3, 0.4)),
                BackgroundColor(Color::srgb(0.08, 0.08, 0.12)),
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

                // ── Player blip ──────────────────────────────────────────────
                map.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(8.0),
                        height: Val::Px(8.0),
                        left: Val::Percent(50.0),
                        top: Val::Percent(50.0),
                        margin: UiRect {
                            left: Val::Px(-4.0),
                            top: Val::Px(-4.0),
                            ..Default::default()
                        },
                        border_radius: BorderRadius::all(Val::Percent(50.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(1.0, 1.0, 1.0)),
                    MinimapPlayerBlip,
                ));

                // ── Enemy blip pool ───────────────────────────────────────────
                for i in 0..MAX_MINIMAP_BLIPS {
                    map.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Px(5.0),
                            height: Val::Px(5.0),
                            // Hidden off-map by default
                            left: Val::Px(-100.0),
                            top: Val::Px(-100.0),
                            border_radius: BorderRadius::all(Val::Percent(50.0)),
                            ..Default::default()
                        },
                        BackgroundColor(Color::srgb(0.9, 0.2, 0.2)),
                        MinimapEnemyBlip { index: i },
                    ));
                }
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

/// Updates enemy blip positions on the minimap every frame.
#[allow(
    clippy::cast_precision_loss,
    clippy::type_complexity
)]
pub fn update_minimap_enemy_blips(
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<(&Transform, Option<&EnemyKind>), With<Enemy>>,
    mut blip_query: Query<(&MinimapEnemyBlip, &mut Node, &mut BackgroundColor)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();

    // Collect visible enemies, capped at the pool size
    let enemies: Vec<(Vec2, Color)> = enemy_query
        .iter()
        .take(MAX_MINIMAP_BLIPS)
        .map(|(t, opt_kind)| {
            let color = opt_kind.map_or(Color::srgb(0.9, 0.2, 0.2), |k| k.color());
            (t.translation.truncate(), color)
        })
        .collect();

    // Minimap panel is 150×150 px. Map MINIMAP_WORLD_RADIUS world-units → 75 px (half of panel).
    let scale = 75.0 / MINIMAP_WORLD_RADIUS;

    for (blip, mut node, mut bg) in &mut blip_query {
        if let Some((world_pos, color)) = enemies.get(blip.index) {
            let delta = *world_pos - player_pos;
            // Clamp so blips on the edge still appear instead of disappearing
            let clamped = delta.clamp_length_max(MINIMAP_WORLD_RADIUS);
            let px = clamped * scale;
            // Panel center is at 75px, blip is 5px wide → offset by 2.5
            let left = 75.0 + px.x - 2.5;
            let top  = 75.0 - px.y - 2.5; // Y is inverted in UI space
            node.left = Val::Px(left);
            node.top  = Val::Px(top);
            *bg = BackgroundColor(*color);
        } else {
            // Hide unused blips off-screen
            node.left = Val::Px(-100.0);
            node.top  = Val::Px(-100.0);
        }
    }
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
