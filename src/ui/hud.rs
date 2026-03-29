use crate::core::components::Health;
use crate::player::active_ability::ActiveAbilityButton;
use crate::player::character::ActiveAbility;
use crate::player::character::SelectedCharacter;
use crate::player::components::{Player, PlayerStats};
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

#[derive(Component)]
pub struct HealthFill;

#[derive(Component)]
pub struct XpFill;

#[derive(Component)]
pub struct LevelText;

#[derive(Component)]
pub struct ActiveAbilityFill;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CircularCooldownMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub progress: f32,
}

impl UiMaterial for CircularCooldownMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/circular_cooldown.wgsl".into()
    }
}

/// Modern, sleek mobile HUD using deeply rounded shapes and premium colors.
#[allow(clippy::too_many_lines)]
pub fn build_mobile_hud(
    mut commands: Commands,
    character: Res<SelectedCharacter>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<CircularCooldownMaterial>>,
) {
    // ── Top Left: Stats ────────────────────────────────────────────────────────
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::VMin(4.0),
            left: Val::VMin(4.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::VMin(1.5),
            ..Default::default()
        })
        .with_children(|root| {
            // Level Badge
            root.spawn((
                Text::new("Lv 1"),
                TextFont {
                    font_size: 20.0,
                    weight: bevy::text::FontWeight::BOLD,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.9, 0.8, 0.2)),
                Node {
                    margin: UiRect::bottom(Val::VMin(0.5)),
                    ..Default::default()
                },
                LevelText,
            ));

            // Health Pill Track
            root.spawn((
                Node {
                    width: Val::VMin(35.0),
                    height: Val::VMin(3.5),
                    border_radius: BorderRadius::MAX,
                    overflow: Overflow::clip(),
                    border: UiRect::all(Val::Px(2.0)),
                    ..Default::default()
                },
                BorderColor::all(Color::srgba(0.8, 0.1, 0.2, 0.7)),
                BackgroundColor(Color::srgba(0.1, 0.0, 0.05, 0.85)),
            ))
            .with_children(|track| {
                track.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        border_radius: BorderRadius::MAX,
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.9, 0.2, 0.3)), // Vibrant Red
                    HealthFill,
                ));
            });

            // XP Pill Track
            root.spawn((
                Node {
                    width: Val::VMin(35.0),
                    height: Val::VMin(2.0),
                    border_radius: BorderRadius::MAX,
                    overflow: Overflow::clip(),
                    border: UiRect::all(Val::Px(1.5)),
                    ..Default::default()
                },
                BorderColor::all(Color::srgba(0.3, 0.1, 0.7, 0.7)),
                BackgroundColor(Color::srgba(0.05, 0.0, 0.1, 0.85)),
            ))
            .with_children(|track| {
                track.spawn((
                    Node {
                        width: Val::Percent(0.0),
                        height: Val::Percent(100.0),
                        border_radius: BorderRadius::MAX,
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.4, 0.2, 1.0)), // Deep Purple XP
                    XpFill,
                ));
            });
        });

    // ── Bottom Right: Touch Abilities ─────────────────────────────────────────
    let char_color = character.accent_color();
    let cooldown_mat = materials.add(CircularCooldownMaterial {
        color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
        progress: 1.0,
    });

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::VMin(6.0),
            right: Val::VMin(6.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::FlexEnd,
            column_gap: Val::VMin(4.0),
            ..Default::default()
        })
        .with_children(|root| {
            let icon_path = match *character {
                SelectedCharacter::Mage | SelectedCharacter::Archer => "ui/blink_icon.png",
                SelectedCharacter::Warlock => "ui/nova_icon.png",
            };

            root.spawn((
                Button,
                ImageNode::new(asset_server.load(icon_path)),
                Node {
                    width: Val::VMin(16.0),
                    height: Val::VMin(16.0),
                    border_radius: BorderRadius::MAX,
                    border: UiRect::all(Val::Px(3.0)),
                    overflow: Overflow::clip(),
                    ..Default::default()
                },
                BorderColor::all(char_color),
                BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.9)),
                ActiveAbilityButton,
            ))
            .with_children(|btn| {
                btn.spawn((
                    MaterialNode(cooldown_mat),
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..Default::default()
                    },
                    ActiveAbilityFill,
                ));
            });
        });
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn update_mobile_hud(
    player_query: Query<(&Health, &PlayerStats, &ActiveAbility, &Player)>,
    mut hp_query: Query<
        &mut Node,
        (
            With<HealthFill>,
            Without<XpFill>,
            Without<ActiveAbilityFill>,
        ),
    >,
    mut xp_query: Query<
        &mut Node,
        (
            With<XpFill>,
            Without<HealthFill>,
            Without<ActiveAbilityFill>,
        ),
    >,
    mut level_text_query: Query<&mut Text, With<LevelText>>,
    active_fill_query: Query<
        &MaterialNode<CircularCooldownMaterial>,
        (
            With<ActiveAbilityFill>,
            Without<HealthFill>,
            Without<XpFill>,
        ),
    >,
    mut materials: ResMut<Assets<CircularCooldownMaterial>>,
) {
    let Ok((health, stats, active_ability, _player)) = player_query.single() else {
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

    // Cooldown overlay: progress is 0.0 (ready) or up to 1.0 (just started)
    // The shader masks based on this value.
    if let Ok(handle) = active_fill_query.single()
        && let Some(mat) = materials.get_mut(handle)
    {
        mat.progress = 1.0 - active_ability.cooldown.fraction();
    }
}
