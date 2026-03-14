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

    // Joystick is initially hidden.
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                display: Display::None,
                width: base_size,
                height: base_size,
                border_radius: BorderRadius::MAX,
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
                    border_radius: BorderRadius::MAX,
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
    mut base_query: Query<&mut Node, (With<JoystickBase>, Without<JoystickKnob>)>,
    mut knob_query: Query<&mut Node, (With<JoystickKnob>, Without<JoystickBase>)>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let window_size = Vec2::new(window.resolution.width(), window.resolution.height());
    let vmin = window_size.x.min(window_size.y);
    let pixel_radius = vmin * (JOYSTICK_VMIN / 100.0);

    for event in touch_events.read() {
        match event.phase {
            bevy::input::touch::TouchPhase::Started => {
                // Only respond if no finger is tracked yet.
                if finger.touch_id.is_none() {
                    finger.touch_id = Some(event.id);
                    finger.anchor = event.position;

                    if let Ok(mut base_node) = base_query.single_mut() {
                        base_node.display = Display::Flex;
                        // Position the base center at the touch point.
                        // event.position is in logical pixels from top-left.
                        base_node.left = Val::Px(event.position.x - pixel_radius);
                        base_node.top = Val::Px(event.position.y - pixel_radius);
                    }
                }
            }
            bevy::input::touch::TouchPhase::Moved => {
                if finger.touch_id == Some(event.id) {
                    let delta = event.position - finger.anchor;
                    let clamped = if delta.length() > pixel_radius {
                        delta.normalize() * pixel_radius
                    } else {
                        delta
                    };

                    joystick_input.direction = Vec2::new(clamped.x, -clamped.y) / pixel_radius;

                    if let Ok(mut knob_node) = knob_query.single_mut() {
                        // Offset by 50% (center) then add pixel delta converted to VMin
                        let x_offset = (clamped.x / vmin) * 100.0;
                        let y_offset = (clamped.y / vmin) * 100.0;
                        knob_node.left = Val::Percent(50.0 + x_offset);
                        knob_node.top = Val::Percent(50.0 + y_offset);
                    }
                }
            }
            bevy::input::touch::TouchPhase::Ended | bevy::input::touch::TouchPhase::Canceled => {
                if finger.touch_id == Some(event.id) {
                    finger.touch_id = None;
                    finger.anchor = Vec2::ZERO;
                    joystick_input.direction = Vec2::ZERO;

                    if let Ok(mut base_node) = base_query.single_mut() {
                        base_node.display = Display::None;
                    }

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
