use bevy::prelude::*;

use super::character::{ActiveAbility, SelectedCharacter};
use super::components::{Player, PlayerStats};
use crate::combat::components::{Orb, Spell};
use crate::combat::nova::NovaEvent;
use crate::core::components::Health;
use crate::enemy::components::Enemy;
use crate::ui::JoystickInput;

/// HUD component for the active ability cooldown bar.
#[derive(Component)]
pub struct ActiveAbilityCooldownFill;

#[derive(Component)]
pub struct ActiveAbilityLabel;

// ── Trigger system ────────────────────────────────────────────────────────────

/// Reads Space bar (PC) / on-screen button inputs. Dispatches the correct
/// active ability based on the `ActiveAbility.kind` on the player.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn trigger_active_ability(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    joystick: Res<JoystickInput>,
    mut player_query: Query<(
        Entity,
        &mut Transform,
        &mut ActiveAbility,
        &mut Player,
        &PlayerStats,
        &mut Health,
    )>,
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform, &mut Health), (With<Enemy>, Without<Player>)>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut ev_nova: MessageWriter<NovaEvent>,
) {
    let Ok((
        player_entity,
        mut player_transform,
        mut ability,
        player,
        stats,
        mut player_health,
    )) = player_query.single_mut()
    else {
        return;
    };

    ability.cooldown.tick(time.delta());

    if !ability.cooldown.is_finished() {
        return;
    }

    let pressed = keyboard.just_pressed(KeyCode::KeyQ);
    if !pressed {
        return;
    }

    // Reset cooldown
    ability.cooldown.reset();

    match ability.kind {
        SelectedCharacter::Mage => {
            // Blink — teleport 320 px in the current facing / joystick direction
            let dir = if joystick.direction.length_squared() > 0.01 {
                joystick.direction.normalize()
            } else {
                player.facing_direction
            };
            player_transform.translation += dir.extend(0.0) * 320.0;
        }

        SelectedCharacter::Archer => {
            // Arrow Rain — fire 8 orbs evenly spaced around the player
            let texture_handle = asset_server.load("HumansProjectiles.png");
            let layout =
                TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 5, None, None);
            let layout_handle = texture_atlases.add(layout);

            for i in 0..8_u16 {
                let angle = std::f32::consts::TAU * (f32::from(i) / 8.0);
                let dir = Vec2::new(angle.cos(), angle.sin());
                commands.spawn((
                    Sprite {
                        image: texture_handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: layout_handle.clone(),
                            index: 10,
                        }),
                        ..Default::default()
                    },
                    Transform {
                        translation: player_transform.translation,
                        scale: Vec3::splat(4.0),
                        ..Default::default()
                    },
                    Orb { direction: dir },
                    Spell {
                        damage: 20.0 * stats.damage_multiplier,
                    },
                ));
            }
        }

        SelectedCharacter::Warlock => {
            // Void Nova — same AoE as Nova but also heals the Warlock
            ev_nova.write(NovaEvent {
                origin: player_transform.translation,
            });
            // Heal 8 HP (capped at max health)
            player_health.0 = (player_health.0 + 8.0).min(stats.max_health);
            let _ = (player_entity, enemy_query, commands); // suppress unused warnings
        }
    }
}

// ── Active-ability HUD ────────────────────────────────────────────────────────

pub fn spawn_active_ability_hud(
    mut commands: Commands,
    character: Res<SelectedCharacter>,
) {
    let label = match *character {
        SelectedCharacter::Mage    => "BLINK [Q]",
        SelectedCharacter::Archer  => "ARROW RAIN [Q]",
        SelectedCharacter::Warlock => "VOID NOVA [Q]",
    };
    let color = character.accent_color();

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::VMin(7.0),
            left: Val::Percent(50.0),
            margin: UiRect::left(Val::VMin(-10.0)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::VMin(0.5),
            ..Default::default()
        })
        .with_children(|root| {
            root.spawn((
                Text::new(label),
                TextFont {
                    font_size: 11.0,
                    ..Default::default()
                },
                TextColor(color),
                ActiveAbilityLabel,
            ));
            root.spawn((
                Node {
                    width: Val::VMin(20.0),
                    height: Val::VMin(2.2),
                    border: UiRect::all(Val::Px(2.0)),
                    overflow: Overflow::clip(),
                    ..Default::default()
                },
                BorderColor::all(Color::srgba(
                    color.to_srgba().red * 0.6,
                    color.to_srgba().green * 0.6,
                    color.to_srgba().blue * 0.6,
                    1.0,
                )),
                BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 0.8)),
            ))
            .with_children(|track| {
                track.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..Default::default()
                    },
                    BackgroundColor(color),
                    ActiveAbilityCooldownFill,
                ));
            });
        });
}

pub fn update_active_ability_hud(
    player_query: Query<&ActiveAbility>,
    mut fill_query: Query<(&mut Node, &mut BackgroundColor), With<ActiveAbilityCooldownFill>>,
    character: Res<SelectedCharacter>,
) {
    let Ok(ability) = player_query.single() else {
        return;
    };
    let pct = (ability.cooldown.fraction() * 100.0).clamp(0.0, 100.0);
    let base_color = character.accent_color();
    if let Ok((mut node, mut bg)) = fill_query.single_mut() {
        node.width = Val::Percent(pct);
        *bg = if ability.cooldown.is_finished() {
            BackgroundColor(base_color)
        } else {
            let s = base_color.to_srgba();
            BackgroundColor(Color::srgba(s.red * 0.4, s.green * 0.4, s.blue * 0.4, 1.0))
        };
    }
}
