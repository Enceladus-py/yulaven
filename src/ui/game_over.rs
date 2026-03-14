use crate::{
    GameState,
    player::components::{Player, PlayerStats},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct GameOverMenu;

#[derive(Component)]
pub struct RestartButton;

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
                        width: Val::VMin(35.0),
                        height: Val::VMin(10.0),
                        margin: UiRect::all(Val::VMin(4.0)),
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
    enemy_query: Query<Entity, With<crate::enemy::components::Enemy>>,
    gem_query: Query<Entity, With<crate::map::components::ExperienceGem>>,
    spell_query: Query<Entity, With<crate::combat::components::Spell>>,
    mut player_query: Query<(
        &mut Player,
        &mut crate::core::components::Health,
        &mut crate::player::components::PlayerStats,
        &mut Transform,
    )>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);

            for entity in &menu_query {
                commands.entity(entity).despawn();
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
                player_query.single_mut()
            {
                health.0 = 100.0;
                player
                    .fireball_timer
                    .set_duration(std::time::Duration::from_secs_f32(0.4));
                player
                    .orb_timer
                    .set_duration(std::time::Duration::from_secs_f32(1.5));
                *stats = PlayerStats::default();
                player.orb_charges = 0;
                transform.translation = Vec3::ZERO;
            }
        }
    }
}
