use bevy::prelude::*;

use crate::GameState;
use crate::core::components::DespawnNextFrame;
use crate::player::character::SelectedCharacter;

#[derive(Component)]
pub struct CharacterSelectMenu;

#[derive(Component)]
pub struct CharacterSelectButton {
    pub character: SelectedCharacter,
}

/// Marker for the heart icon image in each card
#[derive(Component)]
struct HeartIcon;

pub fn spawn_character_select(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the heart icon asset
    let heart_handle = asset_server.load("ui/heart_icon.png");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::VMin(2.0),
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.04, 0.02, 0.08)),
            GlobalZIndex(500),
            CharacterSelectMenu,
        ))
        .with_children(|root| {
            // Title with decorative line
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::VMin(2.0)),
                ..Default::default()
            })
            .with_children(|title_section| {
                // Decorative top line
                title_section.spawn((
                    Node {
                        width: Val::VMin(15.0),
                        height: Val::Px(2.0),
                        margin: UiRect::bottom(Val::VMin(1.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgba(1.0, 0.85, 0.2, 0.5)),
                ));

                // Title
                title_section.spawn((
                    Text::new("YULAVEN"),
                    TextFont {
                        font_size: 48.0,
                        ..Default::default()
                    },
                    TextColor(Color::srgb(1.0, 0.85, 0.2)),
                ));

                // Subtitle
                title_section.spawn((
                    Text::new("Choose Your Hero"),
                    TextFont {
                        font_size: 18.0,
                        ..Default::default()
                    },
                    TextColor(Color::srgb(0.6, 0.55, 0.75)),
                    Node {
                        margin: UiRect::top(Val::VMin(0.5)),
                        ..Default::default()
                    },
                ));
            });

            // Cards row
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::VMin(2.5),
                align_items: AlignItems::Stretch,
                justify_content: JustifyContent::Center,
                ..Default::default()
            })
            .with_children(|row| {
                spawn_hero_card(
                    row,
                    SelectedCharacter::Mage,
                    &asset_server,
                    &mut texture_atlases,
                    &heart_handle,
                );
                spawn_hero_card(
                    row,
                    SelectedCharacter::Archer,
                    &asset_server,
                    &mut texture_atlases,
                    &heart_handle,
                );
                spawn_hero_card(
                    row,
                    SelectedCharacter::Warlock,
                    &asset_server,
                    &mut texture_atlases,
                    &heart_handle,
                );
            });

            // Hint text
            root.spawn((
                Text::new("Tap to select"),
                TextFont {
                    font_size: 14.0,
                    ..Default::default()
                },
                TextColor(Color::srgba(0.5, 0.45, 0.65, 0.7)),
                Node {
                    margin: UiRect::top(Val::VMin(2.0)),
                    ..Default::default()
                },
            ));
        });
}

#[allow(clippy::too_many_lines)]
fn spawn_hero_card(
    parent: &mut ChildSpawnerCommands,
    character: SelectedCharacter,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlasLayout>,
    heart_handle: &Handle<Image>,
) {
    let def = character.definition();
    let accent = def.accent_color;
    let bg = def.card_color;

    // Load the character sprite (no-outline version for cleaner UI)
    let sprite_path = def.sprite_path.replace("outline/", "no-outline/");
    let sprite_handle = asset_server.load(&sprite_path);

    // Create a texture atlas matching the 12x12 grid of 32x32 frames
    let atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 12, 12, None, None);
    let atlas_handle = texture_atlases.add(atlas_layout);

    // Calculate health hearts count
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    let health_count = (def.base_health / 50.0).ceil() as usize;
    let max_hearts = 3;

    // Card container with glow effect
    parent
        .spawn((
            Button,
            Node {
                width: Val::VMin(28.0),
                min_height: Val::VMin(38.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::VMin(1.5)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::VMin(1.5)),
                row_gap: Val::VMin(0.8),
                ..Default::default()
            },
            BorderColor::all(accent),
            BackgroundColor(bg),
            CharacterSelectButton { character },
        ))
        .with_children(|card| {
            // Accent color bar at top
            card.spawn((
                Node {
                    width: Val::Percent(80.0),
                    height: Val::Px(3.0),
                    border_radius: BorderRadius::all(Val::Px(2.0)),
                    margin: UiRect::bottom(Val::VMin(0.5)),
                    ..Default::default()
                },
                BackgroundColor(accent),
            ));

            // Character sprite preview
            card.spawn((
                ImageNode {
                    image: sprite_handle,
                    texture_atlas: Some(TextureAtlas {
                        layout: atlas_handle,
                        index: 0,
                    }),
                    ..Default::default()
                },
                Node {
                    width: Val::VMin(14.0),
                    height: Val::VMin(14.0),
                    margin: UiRect::bottom(Val::VMin(0.5)),
                    ..Default::default()
                },
            ));

            // Name with accent
            card.spawn((
                Text::new(def.display_name),
                TextFont {
                    font_size: 24.0,
                    ..Default::default()
                },
                TextColor(accent),
            ));

            // Health indicator with heart icons
            card.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(4.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::top(Val::VMin(0.3)),
                ..Default::default()
            })
            .with_children(|hearts_row| {
                for i in 0..max_hearts {
                    if i < health_count {
                        // Filled heart
                        hearts_row.spawn((
                            ImageNode {
                                image: heart_handle.clone(),
                                color: Color::srgb(0.9, 0.25, 0.25),
                                ..Default::default()
                            },
                            Node {
                                width: Val::Px(18.0),
                                height: Val::Px(18.0),
                                ..Default::default()
                            },
                            HeartIcon,
                        ));
                    } else {
                        // Empty heart (outline effect using dim color)
                        hearts_row.spawn((
                            ImageNode {
                                image: heart_handle.clone(),
                                color: Color::srgba(0.4, 0.2, 0.2, 0.4),
                                ..Default::default()
                            },
                            Node {
                                width: Val::Px(18.0),
                                height: Val::Px(18.0),
                                ..Default::default()
                            },
                            HeartIcon,
                        ));
                    }
                }
            });

            // Stat bars section
            spawn_stat_bar(
                card,
                "HP",
                def.base_health,
                150.0,
                Color::srgb(0.9, 0.3, 0.3),
            );
            spawn_stat_bar(
                card,
                "SPD",
                def.base_speed_multiplier,
                1.5,
                Color::srgb(0.3, 0.8, 0.4),
            );
            spawn_stat_bar(
                card,
                "DMG",
                def.base_damage_multiplier,
                2.0,
                Color::srgb(0.9, 0.6, 0.2),
            );
            spawn_stat_bar(
                card,
                "RNG",
                def.base_attack_range,
                700.0,
                Color::srgb(0.3, 0.5, 0.9),
            );

            // Divider line
            card.spawn((
                Node {
                    width: Val::Percent(80.0),
                    height: Val::Px(1.0),
                    margin: UiRect::all(Val::VMin(0.3)),
                    ..Default::default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.15)),
            ));

            // Ability section
            card.spawn((
                Text::new("★ PASSIVE"),
                TextFont {
                    font_size: 10.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.7, 0.65, 0.85)),
            ));

            // Passive description
            card.spawn((
                Text::new(def.passive_description),
                TextFont {
                    font_size: 11.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.75, 0.75, 0.85)),
                Node {
                    margin: UiRect::horizontal(Val::VMin(1.0)),
                    ..Default::default()
                },
            ));

            // Active ability
            card.spawn((
                Text::new("⚡ ACTIVE"),
                TextFont {
                    font_size: 10.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.9, 0.75, 0.2)),
                Node {
                    margin: UiRect::top(Val::VMin(0.5)),
                    ..Default::default()
                },
            ));

            card.spawn((
                Text::new(def.active_description),
                TextFont {
                    font_size: 11.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.8, 0.75, 0.5)),
                Node {
                    margin: UiRect::horizontal(Val::VMin(1.0)),
                    ..Default::default()
                },
            ));
        });
}

/// Helper to spawn a stat bar with label and fill
fn spawn_stat_bar(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    value: f32,
    max_value: f32,
    bar_color: Color,
) {
    let fill_percent = (value / max_value).clamp(0.05, 1.0);

    parent
        .spawn(Node {
            width: Val::Percent(85.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(6.0),
            ..Default::default()
        })
        .with_children(|row| {
            // Label
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: 10.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.7)),
                Node {
                    width: Val::Px(30.0),
                    ..Default::default()
                },
            ));

            // Bar background
            row.spawn(Node {
                flex_grow: 1.0,
                height: Val::Px(8.0),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                overflow: Overflow::clip(),
                ..Default::default()
            })
            .with_children(|bar| {
                // Fill
                bar.spawn((
                    Node {
                        width: Val::Percent(fill_percent * 100.0),
                        height: Val::Percent(100.0),
                        border_radius: BorderRadius::all(Val::Px(4.0)),
                        ..Default::default()
                    },
                    BackgroundColor(bar_color),
                ));
            });
        });
}

/// Handle button presses — set `SelectedCharacter` resource and transition to `Playing`.
#[allow(clippy::type_complexity)]
pub fn handle_character_select(
    interaction_query: Query<
        (&Interaction, &CharacterSelectButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut selected: ResMut<SelectedCharacter>,
    mut next_state: ResMut<NextState<GameState>>,
    menu_query: Query<Entity, With<CharacterSelectMenu>>,
    mut commands: Commands,
) {
    for (interaction, btn) in &interaction_query {
        if *interaction == Interaction::Pressed {
            *selected = btn.character;
            next_state.set(GameState::Playing);

            for entity in &menu_query {
                commands.entity(entity).insert(DespawnNextFrame);
            }
        }
    }
}

/// Highlight card on hover with smooth color transition.
#[allow(clippy::type_complexity)]
pub fn highlight_character_card(
    mut query: Query<
        (
            &Interaction,
            &CharacterSelectButton,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, btn, mut bg, mut border) in &mut query {
        let base = btn.character.card_color();
        let accent = btn.character.accent_color();

        if interaction == &Interaction::Hovered {
            let s = base.to_srgba();
            *bg = BackgroundColor(Color::srgb(
                (s.red + 0.08).min(1.0),
                (s.green + 0.08).min(1.0),
                (s.blue + 0.08).min(1.0),
            ));
            *border = BorderColor::all(Color::srgb(
                (accent.to_srgba().red + 0.2).min(1.0),
                (accent.to_srgba().green + 0.2).min(1.0),
                (accent.to_srgba().blue + 0.2).min(1.0),
            ));
        } else {
            *bg = BackgroundColor(base);
            *border = BorderColor::all(accent);
        }
    }
}
