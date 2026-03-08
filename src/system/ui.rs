use bevy::prelude::*; // Added based on user instruction
// Try to use the trait-based approach if ChildBuilder is hard to find
// or find its exact path.

use crate::{
    GameState,
    component::{experience::PlayerStats, health::Health, player::Player},
    system::experience::LevelUpEvent,
};

const PLAYER_MAX_HEALTH: f32 = 100.0;

// ── Marker components ───────────────────────────────────────────────────────

#[derive(Component)]
pub struct LevelUpMenu;

#[derive(Component)]
pub struct UpgradeButton {
    pub skill_id: u32,
}

#[derive(Component)]
pub struct GameOverMenu;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct HealthBarFill;

#[derive(Component)]
pub struct XpBarFill;

#[derive(Component)]
pub struct OrbCooldownFill;

#[derive(Component)]
pub struct FireballChargeFill;

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

// ── HUD ─────────────────────────────────────────────────────────────────────

pub fn spawn_hud(mut commands: Commands) {
    // Root: pinned to bottom-left
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(16.0),
            bottom: Val::Px(16.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..Default::default()
        })
        .with_children(|root| {
            // ── Health bar ──────────────────────────────────────────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                ..Default::default()
            })
            .with_children(|row| {
                row.spawn((
                    Text::new("HP"),
                    TextFont {
                        font_size: 14.0,
                        ..Default::default()
                    },
                    TextColor(Color::srgb(1.0, 0.4, 0.4)),
                ));
                // Background track
                row.spawn((
                    Node {
                        width: Val::Px(160.0),
                        height: Val::Px(14.0),
                        border: UiRect::all(Val::Px(2.0)),
                        overflow: Overflow::clip(),
                        ..Default::default()
                    },
                    BorderColor::all(Color::srgb(0.5, 0.15, 0.15)),
                    BackgroundColor(Color::srgba(0.1, 0.0, 0.0, 0.8)),
                ))
                .with_children(|track| {
                    track.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..Default::default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.85, 0.3)),
                        HealthBarFill,
                    ));
                });
            });

            // ── XP bar ─────────────────────────────────────────────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                ..Default::default()
            })
            .with_children(|row| {
                row.spawn((
                    Text::new("XP"),
                    TextFont {
                        font_size: 14.0,
                        ..Default::default()
                    },
                    TextColor(Color::srgb(0.6, 0.4, 1.0)),
                ));
                // Background track
                row.spawn((
                    Node {
                        width: Val::Px(160.0),
                        height: Val::Px(14.0),
                        border: UiRect::all(Val::Px(2.0)),
                        overflow: Overflow::clip(),
                        ..Default::default()
                    },
                    BorderColor::all(Color::srgb(0.2, 0.1, 0.4)),
                    BackgroundColor(Color::srgba(0.05, 0.0, 0.1, 0.8)),
                ))
                .with_children(|track| {
                    track.spawn((
                        Node {
                            width: Val::Percent(0.0),
                            height: Val::Percent(100.0),
                            ..Default::default()
                        },
                        BackgroundColor(Color::srgb(0.55, 0.25, 1.0)),
                        XpBarFill,
                    ));
                });
            });
        });
}

pub fn update_hud(
    player_query: Query<(&Health, &PlayerStats), With<Player>>,
    mut hp_query: Query<&mut Node, (With<HealthBarFill>, Without<XpBarFill>)>,
    mut xp_query: Query<&mut Node, (With<XpBarFill>, Without<HealthBarFill>)>,
) {
    let Ok((health, stats)) = player_query.single() else {
        return;
    };

    let hp_pct = (health.0 / PLAYER_MAX_HEALTH * 100.0).clamp(0.0, 100.0);
    let xp_pct = (stats.current_xp / stats.required_xp * 100.0).clamp(0.0, 100.0);

    if let Ok(mut node) = hp_query.single_mut() {
        node.width = Val::Percent(hp_pct);
    }
    if let Ok(mut node) = xp_query.single_mut() {
        node.width = Val::Percent(xp_pct);
    }
}

// ── Weapon cooldown HUD (bottom-right) ──────────────────────────────────────
#[allow(clippy::too_many_lines)]
pub fn spawn_weapon_hud(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            right: Val::Px(16.0),
            bottom: Val::Px(16.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            ..Default::default()
        })
        .with_children(|root| {
            // ── Orb card ───────────────────────────────────────────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(6.0),
                ..Default::default()
            })
            .with_children(|row| {
                row.spawn((
                    Node {
                        width: Val::Px(12.0),
                        height: Val::Px(12.0),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.55, 1.0)),
                ));
                row.spawn((
                    Text::new("ORB"),
                    TextFont {
                        font_size: 13.0,
                        ..Default::default()
                    },
                    TextColor(Color::WHITE),
                ));
                row.spawn((
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(12.0),
                        border: UiRect::all(Val::Px(2.0)),
                        overflow: Overflow::clip(),
                        ..Default::default()
                    },
                    BorderColor::all(Color::srgb(0.2, 0.5, 1.0)),
                    BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.8)),
                ))
                .with_children(|track| {
                    track.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..Default::default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.55, 1.0)),
                        OrbCooldownFill,
                    ));
                });
            });

            // ── Fireball card ──────────────────────────────────────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(6.0),
                ..Default::default()
            })
            .with_children(|row| {
                row.spawn((
                    Node {
                        width: Val::Px(12.0),
                        height: Val::Px(12.0),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.4, 0.05)),
                ));
                row.spawn((
                    Text::new("FIREBALL"),
                    TextFont {
                        font_size: 13.0,
                        ..Default::default()
                    },
                    TextColor(Color::WHITE),
                ));
                row.spawn((
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(12.0),
                        border: UiRect::all(Val::Px(2.0)),
                        overflow: Overflow::clip(),
                        ..Default::default()
                    },
                    BorderColor::all(Color::srgb(0.5, 0.2, 0.1)),
                    BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.8)),
                ))
                .with_children(|track| {
                    track.spawn((
                        Node {
                            width: Val::Percent(0.0),
                            height: Val::Percent(100.0),
                            ..Default::default()
                        },
                        BackgroundColor(Color::srgb(1.0, 0.4, 0.05)),
                        FireballChargeFill,
                    ));
                });
            });
        });
}

#[allow(clippy::type_complexity)]
pub fn update_weapon_hud(
    player_query: Query<&Player>,
    mut orb_query: Query<
        (&mut Node, &mut BackgroundColor),
        (With<OrbCooldownFill>, Without<FireballChargeFill>),
    >,
    mut fb_query: Query<
        (&mut Node, &mut BackgroundColor),
        (With<FireballChargeFill>, Without<OrbCooldownFill>),
    >,
) {
    let Ok(player) = player_query.single() else {
        return;
    };

    // Orb bar: shows how full the cooldown timer is (full = ready to fire)
    let orb_elapsed = player.orb_timer.elapsed_secs();
    let orb_duration = player.orb_timer.duration().as_secs_f32();
    let orb_pct = (orb_elapsed / orb_duration * 100.0).clamp(0.0, 100.0);

    if let Ok((mut node, _)) = orb_query.single_mut() {
        node.width = Val::Percent(orb_pct);
    }

    // Fireball charge bar: charges / 5, turns bright orange when ready
    let charges = player.orb_charges;
    let charge_pct = f32::from(charges.min(5)) / 5.0 * 100.0;
    if let Ok((mut node, mut bg)) = fb_query.single_mut() {
        node.width = Val::Percent(charge_pct);
        *bg = if charges >= 5 {
            BackgroundColor(Color::srgb(1.0, 0.6, 0.0)) // bright gold = ready
        } else {
            BackgroundColor(Color::srgb(1.0, 0.4, 0.05)) // dim orange = charging
        };
    }
}

// ── Level-up ─────────────────────────────────────────────────────────────────

pub fn transition_to_levelup(
    mut ev_level_up: MessageReader<LevelUpEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in ev_level_up.read() {
        next_state.set(GameState::LevelUp);
    }
}

pub fn spawn_levelup_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            LevelUpMenu,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Choose an Upgrade"),
                TextFont {
                    font_size: 40.0,
                    ..Default::default()
                },
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..Default::default()
                },
            ));

            let labels = ["Faster Fireballs", "Faster Orbs", "Heal +30 HP"];

            for (i, label) in labels.iter().enumerate() {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(300.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.5)),
                        UpgradeButton {
                            skill_id: u32::try_from(i).unwrap(),
                        },
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new(*label),
                            TextFont {
                                font_size: 25.0,
                                ..Default::default()
                            },
                        ));
                    });
            }
        });
}

#[allow(clippy::type_complexity)]
pub fn handle_skill_selection(
    interaction_query: Query<(&Interaction, &UpgradeButton), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    menu_query: Query<Entity, With<LevelUpMenu>>,
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut Health)>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Ok((mut player, mut health)) = player_query.single_mut() {
                if button.skill_id == 0 {
                    let new_duration = player.fireball_timer.duration().as_secs_f32() * 0.8;
                    player
                        .fireball_timer
                        .set_duration(std::time::Duration::from_secs_f32(new_duration));
                } else if button.skill_id == 1 {
                    let new_duration = player.orb_timer.duration().as_secs_f32() * 0.8;
                    player
                        .orb_timer
                        .set_duration(std::time::Duration::from_secs_f32(new_duration));
                } else if button.skill_id == 2 {
                    health.0 += 30.0;
                }
            }

            next_state.set(GameState::Playing);

            for entity in &menu_query {
                commands.entity(entity).despawn();
            }
        }
    }
}

// ── Game over ────────────────────────────────────────────────────────────────

pub fn spawn_gameover_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            GameOverMenu,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: 60.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.9, 0.2, 0.2)),
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.8)),
                    RestartButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Restart"),
                        TextFont {
                            font_size: 30.0,
                            ..Default::default()
                        },
                    ));
                });
        });
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn handle_restart(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    menu_query: Query<Entity, With<GameOverMenu>>,
    mut commands: Commands,
    enemy_query: Query<Entity, With<crate::component::enemy::Enemy>>,
    gem_query: Query<Entity, With<crate::component::experience::ExperienceGem>>,
    spell_query: Query<Entity, With<crate::component::spell::Spell>>,
    mut player_query: Query<(&mut Player, &mut Health, &mut PlayerStats, &mut Transform)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);

            for entity in &menu_query {
                commands.entity(entity).despawn();
            }

            for entity in &enemy_query {
                commands.entity(entity).despawn();
            }
            for entity in &gem_query {
                commands.entity(entity).despawn();
            }
            for entity in &spell_query {
                commands.entity(entity).despawn();
            }

            if let Ok((mut player, mut health, mut stats, mut transform)) =
                player_query.single_mut()
            {
                health.0 = 100.0;
                player
                    .fireball_timer
                    .set_duration(std::time::Duration::from_secs_f32(0.4));
                player
                    .orb_timer
                    .set_duration(std::time::Duration::from_secs_f32(1.5));
                *stats = PlayerStats::default();
                player.orb_charges = 0;
                transform.translation = Vec3::ZERO;
            }
        }
    }
}

pub fn spawn_minimap_hud(mut commands: Commands) {
    // The actual textures will be assigned by `update_map_textures`
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

    // The player's logical chunk coordinate
    let base_x = (px / tile_size).round();
    let base_y = (py / tile_size).round();

    // The sub-chunk offset of the player (-50% to +50%)
    let pct_offset_x = (px - base_x * tile_size) / tile_size * 100.0;
    // Map is Y-up, GUI is Y-down
    let pct_offset_y = -(py - base_y * tile_size) / tile_size * 100.0;

    // Update colors and scroll position
    for (tile, mut node, mut bg) in &mut tile_query {
        let logical_x = base_x as i32 + tile.offset.x;
        let logical_y = base_y as i32 + tile.offset.y;

        let base_seed =
            (logical_x.wrapping_mul(73_856_093_i32)) ^ (logical_y.wrapping_mul(19_349_663_i32));

        let rng_terrain = crate::system::map::pcg_hash(base_seed as u32);

        if rng_terrain < 0.3 {
            *bg = BackgroundColor(Color::srgb(0.2, 0.6, 0.2)); // Grass
        } else if rng_terrain < 0.5 {
            *bg = BackgroundColor(Color::srgb(0.1, 0.4, 0.1)); // Dark Grass
        } else if rng_terrain < 0.7 {
            *bg = BackgroundColor(Color::srgb(0.6, 0.4, 0.2)); // Dirt
        } else if rng_terrain < 0.85 {
            *bg = BackgroundColor(Color::srgb(0.5, 0.5, 0.5)); // Stone
        } else {
            *bg = BackgroundColor(Color::srgb(0.8, 0.7, 0.3)); // Sand
        }

        // 33.333% is the width/height of one tile piece relative to the 3x3 view
        // The blip is at 50% left/top.
        // If tile offset = (0, 0), it should be centered exactly around the blip, slightly shifted
        // by the player's subchunk percentage offset.
        let view_center = 50.0; // center of UI
        let block_radius = 33.333 / 2.0;

        let start_left = view_center - block_radius + (tile.offset.x as f32 * 33.333);
        let start_top = view_center - block_radius + (-tile.offset.y as f32 * 33.333);

        node.left = Val::Percent(start_left - (pct_offset_x * 0.33333));
        node.top = Val::Percent(start_top - (pct_offset_y * 0.33333));
    }
}
