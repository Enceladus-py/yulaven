use bevy::prelude::*;

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
                    BorderColor(Color::srgb(0.5, 0.15, 0.15)),
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
                    BorderColor(Color::srgb(0.2, 0.1, 0.4)),
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
    let Ok((health, stats)) = player_query.get_single() else {
        return;
    };

    let hp_pct = (health.0 / PLAYER_MAX_HEALTH * 100.0).clamp(0.0, 100.0);
    let xp_pct = (stats.current_xp / stats.required_xp * 100.0).clamp(0.0, 100.0);

    if let Ok(mut node) = hp_query.get_single_mut() {
        node.width = Val::Percent(hp_pct);
    }
    if let Ok(mut node) = xp_query.get_single_mut() {
        node.width = Val::Percent(xp_pct);
    }
}

// ── Level-up ────────────────────────────────────────────────────────────────

pub fn transition_to_levelup(
    mut ev_level_up: EventReader<LevelUpEvent>,
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
            if let Ok((mut player, mut health)) = player_query.get_single_mut() {
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
                commands.entity(entity).despawn_recursive();
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
                commands.entity(entity).despawn_recursive();
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
                player_query.get_single_mut()
            {
                health.0 = 100.0;
                player
                    .fireball_timer
                    .set_duration(std::time::Duration::from_secs_f32(0.4));
                player
                    .orb_timer
                    .set_duration(std::time::Duration::from_secs_f32(1.5));
                *stats = PlayerStats::default();
                transform.translation = Vec3::ZERO;
            }
        }
    }
}
