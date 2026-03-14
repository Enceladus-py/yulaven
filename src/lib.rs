#![deny(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

#[cfg(target_os = "android")]
use bevy::{prelude::bevy_main, window::AppLifecycle};

#[cfg(any(target_os = "android", target_os = "ios"))]
use bevy::winit::WinitSettings;

use bevy::prelude::*;

mod combat;
mod constant;
mod core;
mod enemy;
mod map;
mod player;
pub mod ui;

pub use core::state::GameState;

#[cfg(target_os = "android")]
#[bevy_main]
fn main() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("com.beratdalsuna.yulaven"),
    );

    std::panic::set_hook(Box::new(|panic_info| {
        log::error!("PANIC occurred: {}", panic_info);
    }));

    run_game();
}

pub fn run_game() {
    let mut app = App::new();

    #[cfg(any(target_os = "android", target_os = "ios"))]
    app.insert_resource(WinitSettings::mobile());

    #[cfg(target_os = "android")]
    app.add_systems(
        Update,
        handle_lifetime.run_if(any_with_component::<AudioSink>),
    );

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Yulaven".to_string(),
                    #[cfg(any(target_os = "android", target_os = "ios"))]
                    resizable: false,
                    #[cfg(any(target_os = "android", target_os = "ios"))]
                    mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins(core::CorePlugin)
    .add_plugins(player::PlayerPlugin)
    .add_plugins(enemy::EnemyPlugin)
    .add_plugins(combat::CombatPlugin)
    .add_plugins(map::MapPlugin)
    .add_plugins(ui::UiPlugin)
    .add_message::<player::components::LevelUpEvent>()
    .run();
}

// Pause audio when app goes into background and resume when it returns.
// This is handled by the OS on iOS, but not on Android.
#[cfg(target_os = "android")]
fn handle_lifetime(
    mut app_lifecycle_reader: MessageReader<AppLifecycle>,
    music_controller: Single<&AudioSink>,
) {
    for app_lifecycle in app_lifecycle_reader.read() {
        match app_lifecycle {
            AppLifecycle::Idle | AppLifecycle::WillSuspend | AppLifecycle::WillResume => {}
            AppLifecycle::Suspended => music_controller.pause(),
            AppLifecycle::Running => music_controller.play(),
        }
    }
}
