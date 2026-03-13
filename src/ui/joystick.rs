#[cfg(any(target_os = "android", target_os = "ios"))]
use bevy::input::touch::TouchInput;
use bevy::prelude::*;

/// Resource that stores the current movement direction from the virtual joystick.
/// A zero vector means the joystick is idle.
#[derive(Resource, Default)]
pub struct JoystickInput {
    pub direction: Vec2,
}

/// Marker for the joystick base (background circle).
#[derive(Component)]
pub struct JoystickBase;

/// Marker for the joystick knob (movable dot).
#[derive(Component)]
pub struct JoystickKnob;

/// Tracks the finger that is controlling the joystick.
#[derive(Resource, Default)]
pub struct JoystickFinger {
    pub touch_id: Option<u64>,
    /// The screen position where the touch started (center of the knob travel).
    pub anchor: Vec2,
}

/// The radius of the joystick base in logical pixels.
pub const JOYSTICK_RADIUS: f32 = 80.0;

#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn spawn_joystick(mut commands: Commands) {
    let base_size = JOYSTICK_RADIUS * 2.0;
    let knob_size = JOYSTICK_RADIUS * 0.65;
    let knob_offset = JOYSTICK_RADIUS - knob_size / 2.0;

    // Joystick sits in the lower-left quadrant.
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                bottom: Val::Px(150.0),
                margin: UiRect::new(
                    Val::Px(-JOYSTICK_RADIUS),
                    Val::Px(0.0),
                    Val::Px(0.0),
                    Val::Px(-JOYSTICK_RADIUS),
                ),
                width: Val::Px(base_size),
                height: Val::Px(base_size),
                ..default()
            },
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.15)),
            GlobalZIndex(100),
            JoystickBase,
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(knob_size),
                    height: Val::Px(knob_size),
                    left: Val::Px(knob_offset),
                    top: Val::Px(knob_offset),
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.55)),
                JoystickKnob,
            ));
        });
}

/// Processes touch events to update [`JoystickInput`] and move the knob widget.
#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn update_joystick(
    mut touch_events: MessageReader<TouchInput>,
    mut finger: ResMut<JoystickFinger>,
    mut joystick_input: ResMut<JoystickInput>,
    mut knob_query: Query<&mut Node, With<JoystickKnob>>,
    windows: Query<&Window>,
) {
    let window_size = windows.iter().next().map_or(Vec2::new(1280.0, 800.0), |w| {
        Vec2::new(w.resolution.width(), w.resolution.height())
    });

    let center = window_size / 2.0;
    let knob_size = JOYSTICK_RADIUS * 0.65;
    let knob_offset = JOYSTICK_RADIUS - knob_size / 2.0;

    for event in touch_events.read() {
        // Flip Y so that (0,0) is bottom-left, matching our UI layout.
        let pos = Vec2::new(event.position.x, window_size.y - event.position.y);

        match event.phase {
            bevy::input::touch::TouchPhase::Started => {
                // Only respond if no finger is tracked yet, and the touch is near the center.
                if finger.touch_id.is_none() && pos.distance(center) < 200.0 {
                    finger.touch_id = Some(event.id);
                    finger.anchor = center;
                }
            }
            bevy::input::touch::TouchPhase::Moved => {
                if finger.touch_id == Some(event.id) {
                    let delta = pos - finger.anchor;
                    let clamped = if delta.length() > JOYSTICK_RADIUS {
                        delta.normalize() * JOYSTICK_RADIUS
                    } else {
                        delta
                    };

                    joystick_input.direction = clamped / JOYSTICK_RADIUS;

                    if let Ok(mut knob_node) = knob_query.single_mut() {
                        knob_node.left = Val::Px(knob_offset + clamped.x);
                        // UI Y is inverted relative to game Y.
                        knob_node.top = Val::Px(knob_offset - clamped.y);
                    }
                }
            }
            bevy::input::touch::TouchPhase::Ended | bevy::input::touch::TouchPhase::Canceled => {
                if finger.touch_id == Some(event.id) {
                    finger.touch_id = None;
                    finger.anchor = Vec2::ZERO;
                    joystick_input.direction = Vec2::ZERO;

                    // Reset knob to center.
                    if let Ok(mut knob_node) = knob_query.single_mut() {
                        knob_node.left = Val::Px(knob_offset);
                        knob_node.top = Val::Px(knob_offset);
                    }
                }
            }
        }
    }
}
