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
            LevelUpMenu,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Choose an Upgrade"),
                TextFont {
                    font_size: 40.0,
                    ..Default::default()
                },
                Node {
                    margin: UiRect::bottom(Val::VMin(3.0)),
                    ..Default::default()
                },
            ));

            let labels = ["Faster Fireballs", "Faster Orbs", "Heal +30 HP"];

            for (i, label) in labels.iter().enumerate() {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::VMin(45.0),
                            height: Val::VMin(8.0),
                            margin: UiRect::all(Val::VMin(1.5)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
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
                                font_size: 25.0,
                                ..Default::default()
                            },
                        ));
                    });
            }
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
