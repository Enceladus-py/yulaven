use bevy::prelude::*;

use crate::player::character::SelectedCharacter;
use crate::GameState;

#[derive(Component)]
pub struct CharacterSelectMenu;

#[derive(Component)]
pub struct CharacterSelectButton {
    pub character: SelectedCharacter,
}

pub fn spawn_character_select(mut commands: Commands) {
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
            // ── Title ────────────────────────────────────────────────────────
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

            // ── Cards row ────────────────────────────────────────────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::VMin(3.0),
                align_items: AlignItems::Stretch,
                ..Default::default()
            })
            .with_children(|row| {
                spawn_hero_card(row, SelectedCharacter::Mage);
                spawn_hero_card(row, SelectedCharacter::Archer);
                spawn_hero_card(row, SelectedCharacter::Warlock);
            });
        });
}

fn spawn_hero_card(parent: &mut ChildSpawnerCommands, character: SelectedCharacter) {
    let accent = character.accent_color();
    let bg = character.card_color();

    parent
        .spawn((
            Button,
            Node {
                width: Val::VMin(26.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::VMin(2.5)),
                border: UiRect::all(Val::Px(3.0)),
                border_radius: BorderRadius::all(Val::VMin(3.0)),
                row_gap: Val::VMin(1.5),
                ..Default::default()
            },
            BorderColor::all(accent),
            BackgroundColor(bg),
            CharacterSelectButton { character },
        ))
        .with_children(|card| {
            // Portrait placeholder — colored circle
            card.spawn((
                Node {
                    width: Val::VMin(10.0),
                    height: Val::VMin(10.0),
                    border_radius: BorderRadius::all(Val::Percent(50.0)),
                    border: UiRect::all(Val::Px(3.0)),
                    margin: UiRect::bottom(Val::VMin(1.0)),
                    ..Default::default()
                },
                BorderColor::all(accent),
                BackgroundColor(accent),
            ));

            // Name
            card.spawn((
                Text::new(character.display_name()),
                TextFont {
                    font_size: 22.0,
                    ..Default::default()
                },
                TextColor(accent),
            ));

            // Passive
            card.spawn((
                Text::new(format!("- {}", character.passive_description())),
                TextFont {
                    font_size: 12.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.75, 0.75, 0.85)),
                Node {
                    margin: UiRect::top(Val::VMin(0.5)),
                    ..Default::default()
                },
            ));

            // Active
            card.spawn((
                Text::new(format!("> {}", character.active_description())),
                TextFont {
                    font_size: 12.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.85, 0.75, 0.4)),
                Node {
                    margin: UiRect::top(Val::VMin(0.5)),
                    ..Default::default()
                },
            ));

            // CTA hint at card bottom
            card.spawn((
                Text::new("TAP TO SELECT"),
                TextFont {
                    font_size: 10.0,
                    ..Default::default()
                },
                TextColor(Color::srgba(
                    accent.to_srgba().red,
                    accent.to_srgba().green,
                    accent.to_srgba().blue,
                    0.6,
                )),
                Node {
                    margin: UiRect::top(Val::VMin(1.5)),
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
                commands.entity(entity).despawn();
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
