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

pub fn spawn_character_select(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::VMin(3.0),
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.04, 0.02, 0.08)),
            GlobalZIndex(500),
            CharacterSelectMenu,
        ))
        .with_children(|root| {
            // Title
            root.spawn((
                Text::new("YULAVEN"),
                TextFont {
                    font_size: 36.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(1.0, 0.85, 0.2)),
                Node {
                    margin: UiRect::bottom(Val::VMin(1.0)),
                    ..Default::default()
                },
            ));
            root.spawn((
                Text::new("Choose your hero"),
                TextFont {
                    font_size: 16.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.6, 0.55, 0.75)),
                Node {
                    margin: UiRect::bottom(Val::VMin(2.0)),
                    ..Default::default()
                },
            ));

            // Cards row
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::VMin(3.0),
                align_items: AlignItems::Stretch,
                ..Default::default()
            })
            .with_children(|row| {
                spawn_hero_card(row, SelectedCharacter::Mage, &asset_server);
                spawn_hero_card(row, SelectedCharacter::Archer, &asset_server);
                spawn_hero_card(row, SelectedCharacter::Warlock, &asset_server);
            });
        });
}

fn spawn_hero_card(
    parent: &mut ChildSpawnerCommands,
    character: SelectedCharacter,
    asset_server: &AssetServer,
) {
    let def = character.definition();
    let accent = def.accent_color;
    let bg = def.card_color;

    // Load the character sprite (no-outline version for cleaner UI)
    let sprite_path = def.sprite_path.replace("outline/", "no-outline/");
    let sprite_handle = asset_server.load(&sprite_path);

    parent
        .spawn((
            Button,
            Node {
                width: Val::VMin(22.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::VMin(2.0)),
                border: UiRect::all(Val::Px(3.0)),
                border_radius: BorderRadius::all(Val::VMin(2.0)),
                row_gap: Val::VMin(1.0),
                ..Default::default()
            },
            BorderColor::all(accent),
            BackgroundColor(bg),
            CharacterSelectButton { character },
        ))
        .with_children(|card| {
            // Character sprite preview
            card.spawn((
                ImageNode {
                    image: sprite_handle,
                    ..Default::default()
                },
                Node {
                    width: Val::VMin(10.0),
                    height: Val::VMin(10.0),
                    ..Default::default()
                },
            ));

            // Name
            card.spawn((
                Text::new(def.display_name),
                TextFont {
                    font_size: 20.0,
                    ..Default::default()
                },
                TextColor(accent),
            ));

            // Health indicator
            card.spawn((
                Text::new(character.health_indicator()),
                TextFont {
                    font_size: 14.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.9, 0.3, 0.3)),
            ));

            // One-line ability description
            card.spawn((
                Text::new(def.passive_description),
                TextFont {
                    font_size: 10.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.8)),
                Node {
                    margin: UiRect::top(Val::VMin(0.5)),
                    ..Default::default()
                },
            ));
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

/// Highlight card on hover.
#[allow(clippy::type_complexity)]
pub fn highlight_character_card(
    mut query: Query<
        (&Interaction, &CharacterSelectButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, btn, mut bg) in &mut query {
        let base = btn.character.card_color();
        *bg = match interaction {
            Interaction::Hovered => {
                let s = base.to_srgba();
                BackgroundColor(Color::srgb(
                    (s.red + 0.08).min(1.0),
                    (s.green + 0.08).min(1.0),
                    (s.blue + 0.08).min(1.0),
                ))
            }
            _ => BackgroundColor(base),
        };
    }
}
