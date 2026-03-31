use bevy::prelude::*;

use crate::GameState;
use crate::core::components::DespawnNextFrame;
use crate::player::character::SelectedCharacter;

#[derive(Component)]
pub struct CharacterSelectMenu;

#[derive(Component)]
pub struct CharacterRow {
    pub character: SelectedCharacter,
}

#[derive(Component)]
pub struct SelectHeroButton;

#[derive(Resource, Default)]
pub struct SelectedForDetail(pub Option<SelectedCharacter>);

// Detail panel components for text updates
#[derive(Component)]
pub struct DetailPanel;

#[derive(Component)]
pub struct DetailNameText;

#[derive(Component)]
pub struct DetailHpText;

#[derive(Component)]
pub struct DetailSpdText;

#[derive(Component)]
pub struct DetailDmgText;

#[derive(Component)]
pub struct DetailRngText;

#[derive(Component)]
pub struct DetailPassiveText;

#[derive(Component)]
pub struct DetailActiveText;

#[derive(Component)]
pub struct DetailSprite;

#[allow(clippy::too_many_lines)]
pub fn spawn_character_select(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let pixel_font = asset_server.load("fonts/press_start_2p.ttf");

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
            // Title section
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::VMin(3.0)),
                ..Default::default()
            })
            .with_children(|title_section| {
                // Top decorative line
                title_section.spawn((
                    Node {
                        width: Val::VMin(20.0),
                        height: Val::Px(3.0),
                        margin: UiRect::bottom(Val::VMin(1.5)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgba(1.0, 0.85, 0.2, 0.6)),
                ));

                // Title
                title_section.spawn((
                    Text::new("YULAVEN"),
                    TextFont {
                        font: pixel_font.clone(),
                        font_size: 36.0,
                        ..Default::default()
                    },
                    TextColor(Color::srgb(1.0, 0.85, 0.2)),
                ));

                // Subtitle
                title_section.spawn((
                    Text::new("Choose Your Hero"),
                    TextFont {
                        font: pixel_font.clone(),
                        font_size: 12.0,
                        ..Default::default()
                    },
                    TextColor(Color::srgb(0.6, 0.55, 0.75)),
                    Node {
                        margin: UiRect::top(Val::VMin(0.8)),
                        ..Default::default()
                    },
                ));

                // Bottom decorative line
                title_section.spawn((
                    Node {
                        width: Val::VMin(12.0),
                        height: Val::Px(2.0),
                        margin: UiRect::top(Val::VMin(1.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgba(1.0, 0.85, 0.2, 0.3)),
                ));
            });

            // Character rows container
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::VMin(1.5),
                align_items: AlignItems::Center,
                ..Default::default()
            })
            .with_children(|container| {
                spawn_character_row(
                    container,
                    SelectedCharacter::Mage,
                    &asset_server,
                    &mut texture_atlases,
                    &pixel_font,
                );
                spawn_character_row(
                    container,
                    SelectedCharacter::Archer,
                    &asset_server,
                    &mut texture_atlases,
                    &pixel_font,
                );
                spawn_character_row(
                    container,
                    SelectedCharacter::Warlock,
                    &asset_server,
                    &mut texture_atlases,
                    &pixel_font,
                );
            });

            // Detail panel (hidden by default)
            spawn_detail_panel(root, &asset_server, &mut texture_atlases, &pixel_font);

            // Select Hero button (hidden by default)
            root.spawn((
                Button,
                Node {
                    display: Display::None,
                    padding: UiRect::horizontal(Val::VMin(4.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::VMin(1.0)),
                    margin: UiRect::top(Val::VMin(2.0)),
                    ..Default::default()
                },
                BackgroundColor(Color::srgb(0.2, 0.6, 0.3)),
                BorderColor::all(Color::srgb(0.3, 0.8, 0.4)),
                SelectHeroButton,
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("SELECT HERO"),
                    TextFont {
                        font: pixel_font.clone(),
                        font_size: 18.0,
                        ..Default::default()
                    },
                    TextColor(Color::srgb(0.9, 1.0, 0.9)),
                ));
            });

            // Hint text
            root.spawn((
                Text::new("Tap a hero to view details"),
                TextFont {
                    font: pixel_font.clone(),
                    font_size: 8.0,
                    ..Default::default()
                },
                TextColor(Color::srgba(0.5, 0.45, 0.65, 0.6)),
                Node {
                    margin: UiRect::top(Val::VMin(2.0)),
                    ..Default::default()
                },
            ));
        });

    commands.insert_resource(SelectedForDetail(None));
}

#[allow(clippy::too_many_lines)]
fn spawn_character_row(
    parent: &mut ChildSpawnerCommands,
    character: SelectedCharacter,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlasLayout>,
    pixel_font: &Handle<Font>,
) {
    let def = character.definition();
    let accent = def.accent_color;
    let bg = def.card_color;

    let sprite_path = def.sprite_path.replace("outline/", "no-outline/");
    let sprite_handle = asset_server.load(&sprite_path);

    let atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 12, 12, None, None);
    let atlas_handle = texture_atlases.add(atlas_layout);

    // Row button - two columns: sprite + text, both centered
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(85.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::VMin(3.0),
                padding: UiRect::all(Val::VMin(2.0)),
                border: UiRect::all(Val::Px(3.0)),
                border_radius: BorderRadius::all(Val::VMin(1.5)),
                ..Default::default()
            },
            BorderColor::all(accent),
            BackgroundColor(bg),
            CharacterRow { character },
        ))
        .with_children(|row| {
            // Sprite container
            row.spawn((Node {
                width: Val::VMin(18.0),
                height: Val::VMin(18.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_shrink: 0.0,
                ..Default::default()
            },))
                .with_children(|sprite_container| {
                    sprite_container.spawn((
                        ImageNode {
                            image: sprite_handle,
                            texture_atlas: Some(TextureAtlas {
                                layout: atlas_handle,
                                index: 0,
                            }),
                            ..Default::default()
                        },
                        Node {
                            width: Val::VMin(16.0),
                            height: Val::VMin(16.0),
                            ..Default::default()
                        },
                    ));
                });

            // Character name
            row.spawn((
                Text::new(def.display_name),
                TextFont {
                    font: pixel_font.clone(),
                    font_size: 16.0,
                    ..Default::default()
                },
                TextColor(accent),
            ));
        });
}

#[allow(clippy::too_many_lines)]
fn spawn_detail_panel(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlasLayout>,
    pixel_font: &Handle<Font>,
) {
    // Load default sprite
    let sprite_path = "no-outline/MiniMage.png";
    let sprite_handle = asset_server.load(sprite_path);
    let atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 12, 12, None, None);
    let atlas_handle = texture_atlases.add(atlas_layout);

    parent
        .spawn((
            Node {
                display: Display::None,
                width: Val::VMin(60.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::VMin(2.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::VMin(2.0)),
                row_gap: Val::VMin(1.0),
                margin: UiRect::top(Val::VMin(2.0)),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.08, 0.06, 0.12, 0.95)),
            BorderColor::all(Color::srgba(1.0, 0.85, 0.2, 0.4)),
            DetailPanel,
        ))
        .with_children(|panel| {
            // Character name
            panel.spawn((
                Text::new(""),
                TextFont {
                    font: pixel_font.clone(),
                    font_size: 20.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(1.0, 0.85, 0.2)),
                DetailNameText,
            ));

            // Character sprite
            panel.spawn((
                ImageNode {
                    image: sprite_handle,
                    texture_atlas: Some(TextureAtlas {
                        layout: atlas_handle,
                        index: 0,
                    }),
                    ..Default::default()
                },
                Node {
                    width: Val::VMin(24.0),
                    height: Val::VMin(24.0),
                    ..Default::default()
                },
                DetailSprite,
            ));

            // Stats section
            panel.spawn((
                Text::new("STATS"),
                TextFont {
                    font: pixel_font.clone(),
                    font_size: 12.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.7)),
                Node {
                    margin: UiRect::top(Val::VMin(0.5)),
                    ..Default::default()
                },
            ));

            // HP stat
            panel.spawn((
                Text::new(""),
                TextFont {
                    font: pixel_font.clone(),
                    font_size: 10.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.75, 0.75, 0.85)),
                DetailHpText,
            ));

            // Speed stat
            panel.spawn((
                Text::new(""),
                TextFont {
                    font: pixel_font.clone(),
                    font_size: 10.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.75, 0.75, 0.85)),
                DetailSpdText,
            ));

            // Damage stat
            panel.spawn((
                Text::new(""),
                TextFont {
                    font: pixel_font.clone(),
                    font_size: 10.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.75, 0.75, 0.85)),
                DetailDmgText,
            ));

            // Range stat
            panel.spawn((
                Text::new(""),
                TextFont {
                    font: pixel_font.clone(),
                    font_size: 10.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.75, 0.75, 0.85)),
                DetailRngText,
            ));

            // Passive ability
            panel.spawn((
                Text::new("PASSIVE"),
                TextFont {
                    font: pixel_font.clone(),
                    font_size: 12.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.4, 0.6, 1.0)),
                Node {
                    margin: UiRect::top(Val::VMin(0.5)),
                    ..Default::default()
                },
            ));

            panel.spawn((
                Text::new(""),
                TextFont {
                    font: pixel_font.clone(),
                    font_size: 9.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.8)),
                DetailPassiveText,
            ));

            // Active ability
            panel.spawn((
                Text::new("ACTIVE"),
                TextFont {
                    font: pixel_font.clone(),
                    font_size: 12.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.2)),
                Node {
                    margin: UiRect::top(Val::VMin(0.5)),
                    ..Default::default()
                },
            ));

            panel.spawn((
                Text::new(""),
                TextFont {
                    font: pixel_font.clone(),
                    font_size: 9.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.8)),
                DetailActiveText,
            ));
        });
}

/// Handle character row clicks - show detail panel
#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn handle_character_select(
    interaction_query: Query<(&Interaction, &CharacterRow), (Changed<Interaction>, With<Button>)>,
    mut panel_queries: ParamSet<(
        Query<(Entity, &mut Node, &mut BorderColor), With<DetailPanel>>,
        Query<&mut Node, With<SelectHeroButton>>,
    )>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<DetailNameText>>,
        Query<&mut Text, With<DetailHpText>>,
        Query<&mut Text, With<DetailSpdText>>,
        Query<&mut Text, With<DetailDmgText>>,
        Query<&mut Text, With<DetailRngText>>,
        Query<&mut Text, With<DetailPassiveText>>,
        Query<&mut Text, With<DetailActiveText>>,
    )>,
    mut sprite_query: Query<&mut ImageNode, With<DetailSprite>>,
    mut selected_for_detail: ResMut<SelectedForDetail>,
    _texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, row) in &interaction_query {
        if *interaction == Interaction::Pressed {
            selected_for_detail.0 = Some(row.character);
            let def = row.character.definition();

            // Show detail panel and update border
            for (_entity, mut node, mut border) in &mut panel_queries.p0() {
                node.display = Display::Flex;
                *border = BorderColor::all(def.accent_color);
            }

            // Update name
            for mut text in &mut text_queries.p0() {
                text.0 = def.display_name.to_string();
            }

            // Update stats
            for mut text in &mut text_queries.p1() {
                text.0 = format!("HP: {:.0}", def.base_health);
            }
            for mut text in &mut text_queries.p2() {
                text.0 = format!("SPD: {:.2}x", def.base_speed_multiplier);
            }
            for mut text in &mut text_queries.p3() {
                text.0 = format!("DMG: {:.2}x", def.base_damage_multiplier);
            }
            for mut text in &mut text_queries.p4() {
                text.0 = format!("RNG: {:.0}", def.base_attack_range);
            }

            // Update abilities
            for mut text in &mut text_queries.p5() {
                text.0 = def.passive_description.to_string();
            }
            for mut text in &mut text_queries.p6() {
                text.0 = def.active_description.to_string();
            }

            // Update sprite
            let sprite_path = def.sprite_path.replace("outline/", "no-outline/");
            for mut image in &mut sprite_query {
                image.image = asset_server.load(&sprite_path);
            }

            // Show select button
            for mut node in &mut panel_queries.p1() {
                node.display = Display::Flex;
            }
        }
    }
}

/// Handle select hero button
#[allow(clippy::type_complexity)]
pub fn handle_select_hero(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<SelectHeroButton>)>,
    selected_for_detail: Res<SelectedForDetail>,
    mut selected: ResMut<SelectedCharacter>,
    mut next_state: ResMut<NextState<GameState>>,
    menu_query: Query<Entity, With<CharacterSelectMenu>>,
    mut commands: Commands,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed
            && let Some(char) = selected_for_detail.0
        {
            *selected = char;
            next_state.set(GameState::Playing);
            for entity in &menu_query {
                commands.entity(entity).insert(DespawnNextFrame);
            }
        }
    }
}

/// Highlight character rows on hover
#[allow(clippy::type_complexity)]
pub fn highlight_character_card(
    mut query: Query<
        (
            &Interaction,
            &CharacterRow,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, row, mut bg, mut border) in &mut query {
        let base = row.character.card_color();
        let accent = row.character.accent_color();

        if *interaction == Interaction::Hovered {
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
