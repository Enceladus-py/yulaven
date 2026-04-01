use crate::GameState;
use crate::constant::PIXEL_FONT_PATH;
use crate::core::components::DespawnNextFrame;
use bevy::prelude::*;

// ── Marker Components ────────────────────────────────────────────────────────

/// The small ⏸ button rendered in the HUD during gameplay.
#[derive(Component)]
pub struct PauseButton;

/// Root node of the full-screen pause overlay.
#[derive(Component)]
pub struct PauseMenu;

/// The "Continue" button inside the pause menu.
#[derive(Component)]
pub struct ContinueButton;

/// The "Exit" button inside the pause menu.
#[derive(Component)]
pub struct ExitButton;

// ── Pause Button (spawned with the HUD) ─────────────────────────────────────

/// Spawns a small pause button in the top-right corner of the screen.
/// Call this from `OnExit(CharacterSelect)` alongside the rest of the HUD.
pub fn spawn_pause_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                top: Val::VMin(3.5),
                right: Val::VMin(3.5),
                width: Val::VMin(11.0),
                height: Val::VMin(11.0),
                border_radius: BorderRadius::all(Val::VMin(2.5)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.05, 0.0, 0.12, 0.82)),
            BorderColor::all(Color::srgba(0.55, 0.2, 1.0, 0.55)),
            GlobalZIndex(50),
            PauseButton,
            crate::ui::InGameUi,
        ))
        .with_children(|btn| {
            btn.spawn((
                ImageNode::new(asset_server.load("ui/pause_icon.png")),
                Node {
                    width: Val::Percent(70.0),
                    height: Val::Percent(70.0),
                    ..Default::default()
                },
            ));
        });
}

// ── System: Detect Pause Button Press ────────────────────────────────────────

pub fn handle_pause_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PauseButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Paused);
        }
    }
}

// ── Pause Menu Overlay ────────────────────────────────────────────────────────

#[allow(clippy::too_many_lines)]
pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(PIXEL_FONT_PATH);

    // Full-screen semi-transparent backdrop
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::VMin(5.0),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.08, 0.88)),
            GlobalZIndex(200),
            PauseMenu,
        ))
        .with_children(|root| {
            // ── Title ──────────────────────────────────────────────────────
            root.spawn((
                Text::new("PAUSED"),
                TextFont {
                    font: font.clone(),
                    font_size: 48.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.75, 0.45, 1.0)),
            ));

            // ── Divider bar ───────────────────────────────────────────────
            root.spawn((
                Node {
                    width: Val::VMin(50.0),
                    height: Val::Px(2.0),
                    ..Default::default()
                },
                BackgroundColor(Color::srgba(0.55, 0.2, 1.0, 0.5)),
            ));

            // ── Continue Button ───────────────────────────────────────────
            root.spawn((
                Button,
                Node {
                    width: Val::VMin(55.0),
                    height: Val::VMin(13.0),
                    border_radius: BorderRadius::all(Val::VMin(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..Default::default()
                },
                BackgroundColor(Color::srgba(0.15, 0.05, 0.35, 0.95)),
                BorderColor::all(Color::srgba(0.55, 0.2, 1.0, 0.8)),
                ContinueButton,
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("CONTINUE"),
                    TextFont {
                        font: font.clone(),
                        font_size: 26.0,
                        ..Default::default()
                    },
                    TextColor(Color::srgb(0.9, 0.75, 1.0)),
                ));
            });

            // ── Exit Button ───────────────────────────────────────────────
            root.spawn((
                Button,
                Node {
                    width: Val::VMin(55.0),
                    height: Val::VMin(13.0),
                    border_radius: BorderRadius::all(Val::VMin(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..Default::default()
                },
                BackgroundColor(Color::srgba(0.3, 0.03, 0.08, 0.95)),
                BorderColor::all(Color::srgba(0.9, 0.2, 0.3, 0.7)),
                ExitButton,
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("EXIT"),
                    TextFont {
                        font: font.clone(),
                        font_size: 26.0,
                        ..Default::default()
                    },
                    TextColor(Color::srgb(1.0, 0.5, 0.55)),
                ));
            });
        });
}

// ── System: Handle Pause Menu Buttons ────────────────────────────────────────

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn handle_pause_menu(
    continue_query: Query<&Interaction, (Changed<Interaction>, With<ContinueButton>)>,
    exit_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    menu_query: Query<Entity, With<PauseMenu>>,
    mut commands: Commands,
    // Entities to clean up on exit
    enemy_query: Query<Entity, With<crate::enemy::components::Enemy>>,
    gem_query: Query<Entity, With<crate::combat::components::ExperienceGem>>,
    spell_query: Query<Entity, With<crate::combat::components::Spell>>,
    player_query: Query<Entity, With<crate::player::components::Player>>,
    hud_query: Query<Entity, With<crate::ui::InGameUi>>,
    terrain_query: Query<Entity, With<crate::map::components::TerrainTile>>,
    mut enemy_spawn_timer: ResMut<crate::enemy::systems::EnemySpawnTimer>,
    mut game_timer: ResMut<crate::enemy::systems::GameTimer>,
) {
    // ── Continue ──────────────────────────────────────────────────────────
    for interaction in &continue_query {
        if *interaction == Interaction::Pressed {
            // Despawn the overlay and resume
            for entity in &menu_query {
                commands.entity(entity).insert(DespawnNextFrame);
            }
            next_state.set(GameState::Playing);
            return;
        }
    }

    // ── Exit to Main Menu ─────────────────────────────────────────────────
    for interaction in &exit_query {
        if *interaction == Interaction::Pressed {
            // Remove the overlay
            for entity in &menu_query {
                commands.entity(entity).insert(DespawnNextFrame);
            }

            // Clean up all in-game entities
            for entity in &enemy_query {
                commands.entity(entity).insert(DespawnNextFrame);
            }
            for entity in &gem_query {
                commands.entity(entity).insert(DespawnNextFrame);
            }
            for entity in &spell_query {
                commands.entity(entity).insert(DespawnNextFrame);
            }
            for entity in &player_query {
                commands.entity(entity).insert(DespawnNextFrame);
            }
            for entity in &terrain_query {
                commands.entity(entity).insert(DespawnNextFrame);
            }

            // Clean up HUD / joystick nodes
            for entity in &hud_query {
                commands.entity(entity).insert(DespawnNextFrame);
            }

            // Reset timers
            *enemy_spawn_timer = crate::enemy::systems::EnemySpawnTimer::default();
            *game_timer = crate::enemy::systems::GameTimer::default();

            next_state.set(GameState::MainMenu);
        }
    }
}

// ── System: Hover Visual Feedback ────────────────────────────────────────────

#[allow(clippy::type_complexity)]
pub fn animate_pause_buttons(
    mut continue_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<ContinueButton>,
            Without<ExitButton>,
        ),
    >,
    mut exit_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<ExitButton>,
            Without<ContinueButton>,
        ),
    >,
) {
    for (interaction, mut color) in &mut continue_query {
        *color = match interaction {
            Interaction::Hovered | Interaction::Pressed => {
                BackgroundColor(Color::srgba(0.25, 0.1, 0.55, 0.98))
            }
            Interaction::None => BackgroundColor(Color::srgba(0.15, 0.05, 0.35, 0.95)),
        };
    }
    for (interaction, mut color) in &mut exit_query {
        *color = match interaction {
            Interaction::Hovered | Interaction::Pressed => {
                BackgroundColor(Color::srgba(0.5, 0.05, 0.12, 0.98))
            }
            Interaction::None => BackgroundColor(Color::srgba(0.3, 0.03, 0.08, 0.95)),
        };
    }
}
