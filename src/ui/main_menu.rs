use bevy::prelude::*;

use crate::GameState;
use crate::core::components::DespawnNextFrame;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct PlayButton;

pub fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::VMin(5.0),
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.04, 0.02, 0.08)),
            GlobalZIndex(500),
            MainMenuUI,
        ))
        .with_children(|root| {
            // Title
            root.spawn((
                Text::new("YULAVEN"),
                TextFont {
                    font_size: 48.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(1.0, 0.85, 0.2)),
                Node {
                    margin: UiRect::bottom(Val::VMin(4.0)),
                    ..Default::default()
                },
            ));

            // Play Button
            root.spawn((
                Button,
                Node {
                    width: Val::VMin(30.0),
                    height: Val::VMin(10.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(3.0)),
                    border_radius: BorderRadius::all(Val::VMin(2.0)),
                    ..Default::default()
                },
                BorderColor::all(Color::srgb(0.3, 0.6, 1.0)),
                BackgroundColor(Color::srgb(0.1, 0.15, 0.35)),
                PlayButton,
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("PLAY"),
                    TextFont {
                        font_size: 24.0,
                        ..Default::default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
}

pub fn handle_main_menu(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    menu_query: Query<Entity, With<MainMenuUI>>,
    mut commands: Commands,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::CharacterSelect);

            for entity in &menu_query {
                commands.entity(entity).insert(DespawnNextFrame);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn highlight_play_button(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
) {
    for (interaction, mut bg) in &mut query {
        *bg = match interaction {
            Interaction::Hovered => BackgroundColor(Color::srgb(0.15, 0.25, 0.5)),
            _ => BackgroundColor(Color::srgb(0.1, 0.15, 0.35)),
        };
    }
}
