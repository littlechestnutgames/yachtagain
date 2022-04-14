#![allow(unused)]
#![feature(main_separator_str)]

mod config;
mod shared;
mod splashscreen;
mod win;
mod mainmenu;

use config::*;
use bevy::prelude::*;
use bevy_audio::*;
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin};
use derive_more::{Deref, DerefMut};
use mainmenu::MainMenuPlugin;
use splashscreen::SplashScreenPlugin;
use win::WindowPlugin;
use shared::AppState;

fn main() {
    let app = App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::TitleScreen)
        .add_plugin(WindowPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(AudioPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(SplashScreenPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
