use crate::{
    GameState,
    core::components::Health,
    player::components::{LevelUpEvent, Player},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct LevelUpMenu;

#[derive(Component)]
pub struct UpgradeButton {
    pub skill_id: u32,
}

pub fn transition_to_levelup(
    mut ev_level_up: MessageReader<LevelUpEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in ev_level_up.read() {
        next_state.set(GameState::LevelUp);
    }
}

pub fn spawn_levelup_menu(mut commands: Commands) {
    // Root container: Full screen overlay to dim the background
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            GlobalZIndex(1000), // Ensure it's above everything
            LevelUpMenu,
        ))
        .with_children(|root| {
            // Card container
            root.spawn((
                Node {
                    width: Val::VMin(80.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::VMin(4.0)),
                    border_radius: BorderRadius::all(Val::VMin(4.0)),
                    ..Default::default()
                },
                BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.95)),
            ))
            .with_children(|card| {
                card.spawn((
                    Text::new("Choose an Upgrade"),
                    TextFont {
                        font_size: 24.0, // Base size, slightly reduced
                        ..Default::default()
                    },
                    Node {
                        margin: UiRect::bottom(Val::VMin(4.0)),
                        ..Default::default()
                    },
                ));

                let labels = ["Faster Fireballs", "Faster Orbs", "Heal +30 HP"];

                for (i, label) in labels.iter().enumerate() {
                    card.spawn((
                        Button,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::VMin(10.0),
                            margin: UiRect::vertical(Val::VMin(1.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(Val::VMin(2.0)),
                            ..Default::default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.5)),
                        UpgradeButton {
                            skill_id: u32::try_from(i).unwrap_or(0),
                        },
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new(*label),
                            TextFont {
                                font_size: 18.0, // Base size, slightly reduced
                                ..Default::default()
                            },
                        ));
                    });
                }
            });
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
            if let Ok((mut player, mut health)) = player_query.single_mut() {
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
                commands.entity(entity).despawn();
            }
        }
    }
}
