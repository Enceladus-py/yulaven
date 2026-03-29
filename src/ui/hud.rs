use crate::core::components::Health;
use crate::player::components::{Player, PlayerStats};
use bevy::prelude::*;

#[derive(Component)]
pub struct HealthBarFill;

#[derive(Component)]
pub struct XpBarFill;

#[derive(Component)]
pub struct OrbCooldownFill;

#[derive(Component)]
pub struct FireballChargeFill;

#[derive(Component)]
pub struct LevelText;

pub fn spawn_hud(mut commands: Commands) {
    // Root: pinned to bottom-left
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::VMin(2.0),
            bottom: Val::VMin(2.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::VMin(1.0),
            ..Default::default()
        })
        .with_children(|root| {
            // ── Level badge ─────────────────────────────────────────────────
            root.spawn((
                Text::new("Lv 1"),
                TextFont {
                    font_size: 18.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(1.0, 0.85, 0.1)),
                Node {
                    margin: UiRect::bottom(Val::VMin(0.5)),
                    ..Default::default()
                },
                LevelText,
            ));

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
                        width: Val::VMin(30.0),
                        height: Val::VMin(2.5),
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
                        width: Val::VMin(30.0),
                        height: Val::VMin(2.5),
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
    mut level_text_query: Query<&mut Text, With<LevelText>>,
) {
    let Ok((health, stats)) = player_query.single() else {
        return;
    };

    let hp_pct = (health.0 / stats.max_health * 100.0).clamp(0.0, 100.0);
    let xp_pct = (stats.current_xp / stats.required_xp * 100.0).clamp(0.0, 100.0);

    if let Ok(mut node) = hp_query.single_mut() {
        node.width = Val::Percent(hp_pct);
    }
    if let Ok(mut node) = xp_query.single_mut() {
        node.width = Val::Percent(xp_pct);
    }
    if let Ok(mut text) = level_text_query.single_mut() {
        **text = format!("Lv {}", stats.level);
    }
}

// ── Weapon cooldown HUD (bottom-right) ──────────────────────────────────────
#[allow(clippy::too_many_lines)]
pub fn spawn_weapon_hud(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            right: Val::VMin(2.0),
            bottom: Val::VMin(2.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::VMin(1.5),
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
                        width: Val::VMin(2.5),
                        height: Val::VMin(2.5),
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
                        width: Val::VMin(20.0),
                        height: Val::VMin(2.2),
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
                        width: Val::VMin(2.5),
                        height: Val::VMin(2.5),
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
                        width: Val::VMin(20.0),
                        height: Val::VMin(2.2),
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
