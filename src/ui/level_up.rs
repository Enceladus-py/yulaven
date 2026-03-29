use crate::{
    GameState,
    core::components::{DespawnNextFrame, Health},
    player::components::{LevelUpEvent, Player, PlayerStats},
};
use bevy::prelude::*;
use rand::seq::SliceRandom;

#[derive(Component)]
pub struct LevelUpMenu;

#[derive(Component)]
pub struct UpgradeButton {
    pub kind: UpgradeKind,
}

/// All possible upgrades the player can receive on level-up.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UpgradeKind {
    // ── Universal stats ──────────────────────────────────────────────────────
    AttackSpeed,
    AttackDamage,
    MoveSpeed,
    MaxHealth,
    Magnet,
    FireRange,
}

impl UpgradeKind {
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::AttackSpeed => "⚡ Attack Speed",
            Self::AttackDamage => "💥 Damage",
            Self::MoveSpeed => "👟 Move Speed",
            Self::MaxHealth => "❤️ Max Health",
            Self::Magnet => "🧲 Gem Magnet",
            Self::FireRange => "🎯 Fire Range",
        }
    }

    #[must_use]
    pub fn description(self) -> &'static str {
        match self {
            Self::AttackSpeed => "+15% attack speed",
            Self::AttackDamage => "+20% spell damage",
            Self::MoveSpeed => "+10% movement speed",
            Self::MaxHealth => "+25 maximum HP",
            Self::Magnet => "+25% gem magnet radius",
            Self::FireRange => "+20% targeting range",
        }
    }

    #[must_use]
    pub fn accent_color(self) -> Color {
        match self {
            Self::AttackSpeed => Color::srgb(0.9, 0.8, 0.1),
            Self::AttackDamage => Color::srgb(1.0, 0.3, 0.1),
            Self::MoveSpeed => Color::srgb(0.2, 0.8, 0.5),
            Self::MaxHealth => Color::srgb(0.2, 0.9, 0.3),
            Self::Magnet => Color::srgb(0.4, 0.5, 1.0),
            Self::FireRange => Color::srgb(0.6, 0.3, 1.0),
        }
    }
}

const ALL_UPGRADES: &[UpgradeKind] = &[
    UpgradeKind::AttackSpeed,
    UpgradeKind::AttackDamage,
    UpgradeKind::MoveSpeed,
    UpgradeKind::MaxHealth,
    UpgradeKind::Magnet,
    UpgradeKind::FireRange,
];

pub fn transition_to_levelup(
    mut ev_level_up: MessageReader<LevelUpEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in ev_level_up.read() {
        next_state.set(GameState::LevelUp);
    }
}

pub fn spawn_levelup_menu(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let mut pool: Vec<UpgradeKind> = ALL_UPGRADES.to_vec();
    pool.shuffle(&mut rng);
    let selected: Vec<UpgradeKind> = pool.into_iter().take(3).collect();

    // Root: full-screen dim overlay
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                row_gap: Val::VMin(2.0),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.05, 0.85)),
            GlobalZIndex(1000),
            LevelUpMenu,
        ))
        .with_children(|root| {
            // Title
            root.spawn((
                Text::new("LEVEL UP!"),
                TextFont {
                    font_size: 32.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(1.0, 0.85, 0.1)),
                Node {
                    margin: UiRect::bottom(Val::VMin(2.0)),
                    ..Default::default()
                },
            ));
            root.spawn((
                Text::new("Choose an upgrade"),
                TextFont {
                    font_size: 16.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.8)),
                Node {
                    margin: UiRect::bottom(Val::VMin(3.0)),
                    ..Default::default()
                },
            ));

            // Cards row
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::VMin(3.0),
                ..Default::default()
            })
            .with_children(|row| {
                for kind in selected {
                    spawn_upgrade_card(row, kind);
                }
            });
        });
}

fn spawn_upgrade_card(parent: &mut ChildSpawnerCommands, kind: UpgradeKind) {
    let accent = kind.accent_color();
    parent
        .spawn((
            Button,
            Node {
                width: Val::VMin(24.0),
                height: Val::VMin(32.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::VMin(2.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::VMin(3.0)),
                row_gap: Val::VMin(1.5),
                ..Default::default()
            },
            BorderColor::all(accent),
            BackgroundColor(Color::srgba(0.1, 0.1, 0.18, 0.95)),
            UpgradeButton { kind },
        ))
        .with_children(|card| {
            // Icon / emoji label
            card.spawn((
                Text::new(kind.label()),
                TextFont {
                    font_size: 18.0,
                    ..Default::default()
                },
                TextColor(accent),
            ));
            // Description
            card.spawn((
                Text::new(kind.description()),
                TextFont {
                    font_size: 13.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.75, 0.75, 0.85)),
                Node {
                    margin: UiRect::top(Val::VMin(0.5)),
                    ..Default::default()
                },
            ));
        });
}

#[allow(clippy::type_complexity)]
pub fn handle_skill_selection(
    interaction_query: Query<(&Interaction, &UpgradeButton), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    menu_query: Query<Entity, With<LevelUpMenu>>,
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut Health, &mut PlayerStats)>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Ok((mut player, mut health, mut stats)) = player_query.single_mut() {
                apply_upgrade(button.kind, &mut player, &mut health, &mut stats);
            }

            next_state.set(GameState::Playing);

            for entity in &menu_query {
                commands.entity(entity).insert(DespawnNextFrame);
            }
        }
    }
}

fn apply_upgrade(
    kind: UpgradeKind,
    player: &mut Player,
    health: &mut Health,
    stats: &mut PlayerStats,
) {
    match kind {
        UpgradeKind::AttackSpeed => {
            let new_fb = player.fireball_timer.duration().as_secs_f32() * 0.85;
            player
                .fireball_timer
                .set_duration(std::time::Duration::from_secs_f32(new_fb.max(0.1)));
            let new_orb = player.orb_timer.duration().as_secs_f32() * 0.85;
            player
                .orb_timer
                .set_duration(std::time::Duration::from_secs_f32(new_orb.max(0.2)));
        }
        UpgradeKind::AttackDamage => {
            stats.damage_multiplier *= 1.20;
        }
        UpgradeKind::MoveSpeed => {
            stats.speed_multiplier *= 1.10;
        }
        UpgradeKind::MaxHealth => {
            stats.max_health += 25.0;
            health.0 = (health.0 + 15.0).min(stats.max_health); // also heal a bit
        }
        UpgradeKind::Magnet => {
            stats.magnet_radius *= 1.25;
        }
        UpgradeKind::FireRange => {
            stats.attack_range *= 1.20;
        }
    }
}
