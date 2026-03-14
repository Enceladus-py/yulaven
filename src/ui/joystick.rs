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

/// Default base radius as a percentage of the smaller screen dimension (`VMin`).
pub const JOYSTICK_VMIN: f32 = 15.0;

#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn spawn_joystick(mut commands: Commands) {
    let base_size = Val::VMin(JOYSTICK_VMIN * 2.0);
    let knob_size = Val::VMin(JOYSTICK_VMIN * 0.65 * 2.0);

    // Joystick sits in the lower-center.
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                bottom: Val::Percent(10.0),
                // We'll use transform/translation for centering instead of margins if possible,
                // but stick to Node layout for now with VMin.
                width: base_size,
                height: base_size,
                // Center the base on (50%, 10%)
                margin: UiRect::new(
                    Val::VMin(-JOYSTICK_VMIN),
                    Val::Px(0.0),
                    Val::Px(0.0),
                    Val::VMin(-JOYSTICK_VMIN),
                ),
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
                    width: knob_size,
                    height: knob_size,
                    // Center the knob in the base
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    margin: UiRect::new(
                        Val::VMin(-JOYSTICK_VMIN * 0.65),
                        Val::Px(0.0),
                        Val::VMin(-JOYSTICK_VMIN * 0.65),
                        Val::Px(0.0),
                    ),
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
    let Ok(window) = windows.single() else {
        return;
    };
    let window_size = Vec2::new(window.resolution.width(), window.resolution.height());
    let vmin = window_size.x.min(window_size.y);
    let pixel_radius = vmin * (JOYSTICK_VMIN / 100.0);

    let center = window_size / 2.0;

    for event in touch_events.read() {
        // Flip Y so that (0,0) is bottom-left, matching our UI layout.
        let pos = Vec2::new(event.position.x, window_size.y - event.position.y);

        match event.phase {
            bevy::input::touch::TouchPhase::Started => {
                // Only respond if no finger is tracked yet, and the touch is near the center.
                if finger.touch_id.is_none() && pos.distance(center) < pixel_radius * 2.0 {
                    finger.touch_id = Some(event.id);
                    finger.anchor = center;
                }
            }
            bevy::input::touch::TouchPhase::Moved => {
                if finger.touch_id == Some(event.id) {
                    let delta = pos - finger.anchor;
                    let clamped = if delta.length() > pixel_radius {
                        delta.normalize() * pixel_radius
                    } else {
                        delta
                    };

                    joystick_input.direction = clamped / pixel_radius;

                    if let Ok(mut knob_node) = knob_query.single_mut() {
                        // Offset by 50% (center) then add pixel delta converted to VMin
                        let x_offset = (clamped.x / vmin) * 100.0;
                        let y_offset = (clamped.y / vmin) * 100.0;
                        knob_node.left = Val::Percent(50.0 + x_offset);
                        // UI Y (top) is inverted relative to screen Y (bottom-up)
                        knob_node.top = Val::Percent(50.0 - y_offset);
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
                        knob_node.left = Val::Percent(50.0);
                        knob_node.top = Val::Percent(50.0);
                    }
                }
            }
        }
    }
}
