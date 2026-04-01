use bevy::prelude::*;

use crate::GameState;
use crate::constant::PIXEL_FONT_PATH;
use crate::core::components::DespawnNextFrame;
use crate::ui::PixelFont;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct MainMenuSprite;

#[derive(Component)]
pub struct LogoNode; // Marker for the lantern sprite

#[derive(Resource, Default)]
pub struct MenuExitTime(pub Option<f32>);

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let logo_handle = asset_server.load("ui/yulaven_lantern_sprites.png");
    let title_handle = asset_server.load("ui/yulaven_title.png");
    let font_handle = asset_server.load(PIXEL_FONT_PATH);
    commands.insert_resource(PixelFont(font_handle));

    // 4x2 grid on a 1024x1024 sheet -> each frame is 256x512
    let layout = TextureAtlasLayout::from_grid(UVec2::new(256, 512), 4, 2, None, None);
    let layout_handle = texture_atlases.add(layout);

    // --- UI Overlay (Interaction & Branding) ---
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            BackgroundColor(Color::NONE), // Explicit transparency for root
            GlobalZIndex(100),
            MainMenuUI,
        ))
        .with_children(|root| {
            // Lantern (Animated Sprite Sheet as UI Image)
            root.spawn((
                ImageNode {
                    image: logo_handle,
                    texture_atlas: Some(TextureAtlas {
                        layout: layout_handle,
                        index: 0,
                    }),
                    ..Default::default()
                },
                Node {
                    width: Val::VMin(45.0), // Responsive size for lantern - INCREASED
                    height: Val::Auto,
                    aspect_ratio: Some(0.5),                  // 256x512 ratio
                    margin: UiRect::bottom(Val::VMin(-15.0)), // Deeper overlap for better framing
                    ..Default::default()
                },
                BackgroundColor(Color::NONE), // Explicit transparency
                LogoNode,
                MainMenuUI,
                Transform::default(),
                Visibility::default(),
            ));

            // Title Logo as a UI Node for responsive scaling
            root.spawn((
                ImageNode::new(title_handle),
                Node {
                    width: Val::VMin(85.0), // Fits well on mobile
                    height: Val::Auto,
                    aspect_ratio: Some(1.0), // The title logo is 1024x1024 square
                    margin: UiRect::bottom(Val::VMin(10.0)),
                    ..Default::default()
                },
                BackgroundColor(Color::NONE), // Explicit transparency
                MainMenuUI,
                Transform::default(),
                Visibility::default(),
            ));

            // Premium Play Button
            root.spawn((
                Button,
                Node {
                    width: Val::VMin(32.0),
                    height: Val::VMin(10.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(3.0)),
                    border_radius: BorderRadius::all(Val::VMin(1.5)),
                    ..Default::default()
                },
                BorderColor::all(Color::srgba(0.9, 0.7, 0.3, 0.9)), // Shimmering Gold
                BackgroundColor(Color::srgba(0.12, 0.04, 0.22, 0.95)), // Dark Arcane Purple
                BoxShadow(vec![ShadowStyle {
                    color: Color::srgba(0.8, 0.2, 1.0, 0.95), // Initial Magenta Pulse
                    blur_radius: Val::VMin(15.0),
                    spread_radius: Val::VMin(4.0),
                    ..Default::default()
                }]),
                PlayButton,
                Transform::default(),
                Visibility::default(),
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("PLAY"),
                    TextFont {
                        font: asset_server.load(PIXEL_FONT_PATH),
                        font_size: 28.0,
                        ..Default::default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
}

pub fn handle_main_menu(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut exit_time: ResMut<MenuExitTime>,
    time: Res<Time>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed && exit_time.0.is_none() {
            exit_time.0 = Some(time.elapsed_secs());
        }
    }
}

pub fn check_menu_exit(
    exit_time: Res<MenuExitTime>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    menu_ui_query: Query<Entity, With<MainMenuUI>>,
    menu_sprite_query: Query<Entity, With<MainMenuSprite>>,
    mut commands: Commands,
) {
    if let Some(start) = exit_time.0
        && time.elapsed_secs() - start > 0.6
    {
        next_state.set(GameState::CharacterSelect);

        for entity in &menu_ui_query {
            commands.entity(entity).insert(DespawnNextFrame);
        }
        for entity in &menu_sprite_query {
            commands.entity(entity).insert(DespawnNextFrame);
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn highlight_play_button(
    mut query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Transform,
            &mut BoxShadow,
        ),
        With<PlayButton>,
    >,
    time: Res<Time>,
    exit_time: Res<MenuExitTime>,
) {
    let t = time.elapsed_secs();
    let pulse = (t * 2.5).sin() * 0.5 + 0.5; // slow breath pulse 0..1

    for (interaction, mut bg, mut border, mut transform, mut shadow) in &mut query {
        let (target_bg, target_border, mut target_scale, target_shadow_color) = match interaction {
            Interaction::Hovered => (
                BackgroundColor(Color::srgba(0.2, 0.08, 0.35, 0.95)), // Vibrant Purple
                BorderColor::all(Color::srgba(1.0, 0.85, 0.4, 1.0)),  // Bright Gold
                Vec3::splat(1.15),
                Color::srgba(1.0, 0.2, 0.8, 0.95), // High-intensity Magenta glow
            ),
            Interaction::Pressed => (
                BackgroundColor(Color::srgba(1.0, 0.9, 0.5, 0.9)),
                BorderColor::all(Color::WHITE),
                Vec3::splat(1.18),
                Color::srgba(1.0, 0.9, 0.2, 1.0), // Golden flash
            ),
            Interaction::None => (
                BackgroundColor(Color::srgba(0.12, 0.04, 0.22, 0.95)), // Base Purple
                BorderColor::all(Color::srgba(0.8, 0.6, 0.25, 0.85)),  // Muted Gold
                Vec3::splat(1.0 + pulse * 0.03),                       // "Breath" pulse
                // Mix between Magenta and Gold for the pulse
                Color::srgba(
                    0.6 + pulse * 0.4, // Shift towards Gold (1.0)
                    0.2 + pulse * 0.6, // Shift towards Gold (0.8)
                    0.9 - pulse * 0.6, // Shift away from Purple (0.3)
                    0.8,
                ),
            ),
        };

        // If exit transition is active: Red Lightning Surge
        if let Some(start) = exit_time.0 {
            let elapsed = time.elapsed_secs() - start;
            let strike_flicker = (time.elapsed_secs() * 35.0).sin() > 0.0;
            let shake = (time.elapsed_secs() * 60.0).cos() * 8.0;

            target_scale = Vec3::splat(1.0 + elapsed * 8.0); // Extreme scale up
            bg.0 = if strike_flicker {
                Color::srgba(1.0, 0.0, 0.0, 1.0)
            } else {
                Color::WHITE
            };
            transform.translation.x += shake;
            transform.translation.y += shake;

            if let Some(primary_shadow) = shadow.0.first_mut() {
                primary_shadow.color = Color::srgba(1.0, 0.0, 0.0, 0.8);
            }
        } else {
            // Normal pulsing bloom color
            if let Some(primary_shadow) = shadow.0.first_mut() {
                primary_shadow.color = primary_shadow
                    .color
                    .mix(&target_shadow_color, time.delta_secs() * 5.0);
                primary_shadow.blur_radius = Val::VMin(10.0 + pulse * 15.0); // Extreme pulsing blur (10-25)
            }
        }

        // Smooth liquid transitions
        bg.0 = bg.0.mix(&target_bg.0, time.delta_secs() * 12.0);

        let lerp_factor = time.delta_secs() * 12.0;
        border.top = border.top.mix(&target_border.top, lerp_factor);
        border.right = border.right.mix(&target_border.right, lerp_factor);
        border.bottom = border.bottom.mix(&target_border.bottom, lerp_factor);
        border.left = border.left.mix(&target_border.left, lerp_factor);

        transform.scale = transform.scale.lerp(target_scale, lerp_factor);
    }
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn animate_burning_logo(
    time: Res<Time>,
    mut logo_query: Query<(&mut Transform, &mut ImageNode), With<LogoNode>>,
    exit_time: Res<MenuExitTime>,
) {
    let t = time.elapsed_secs();

    // Slower animation speed (5fps) for better rhythmic pacing
    let speed = 5.0;
    let raw_frame = (t * speed).floor() as usize;

    // Sequence: frames 0-7 once, then loop 5-7
    let frame = if raw_frame < 8 {
        raw_frame
    } else {
        5 + (raw_frame - 8) % 3
    };

    // Subtle flicker logic for "burning" effect (used ONLY for scale)
    let flicker = (t * 15.0).sin() * 0.03 + 1.0;

    for (mut transform, mut image_node) in &mut logo_query {
        // Vertical bobbing (levitation) relative to its layout position
        let bob = (t * 1.5).sin() * 12.0; // Slower, deeper bob

        // Exit transition: Red Lightning Strike (Flicker, Shake, Extreme Scale)
        if let Some(start) = exit_time.0 {
            let elapsed = time.elapsed_secs() - start;
            let strike_flicker = (t * 45.0).sin() > 0.0;
            let shake_x = (t * 70.0).sin() * 15.0;
            let shake_y = (t * 75.0).cos() * 15.0;

            let burst = 1.0 + elapsed * 10.0; // More aggressive scale
            transform.scale = Vec3::splat(flicker * burst);
            transform.translation.x += shake_x;
            transform.translation.y += shake_y;

            // Flash red
            image_node.color = if strike_flicker {
                Color::srgba(1.0, 0.0, 0.0, 1.0)
            } else {
                Color::WHITE
            };
        } else {
            transform.scale = Vec3::splat(flicker);
            // Ensure full color (original asset colors)
            image_node.color = Color::WHITE;
        }

        transform.translation.y += bob;

        if let Some(atlas) = &mut image_node.texture_atlas {
            atlas.index = frame;
        }
    }
}
