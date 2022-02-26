#![feature(try_blocks)]
#![feature(let_else)]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_egui::EguiPlugin;

mod menus;
mod singleplayer;
mod utils;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    MainMenu,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        // Configure the game window
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 900.0,
            vsync: true,
            mode: WindowMode::Windowed,
            title: "Awesome Bevy Game".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.11, 0.039, 0.004)))
        // Standard Bevy functionality
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(EguiPlugin)
        .add_plugin(menus::MainMenuScene)
        .add_plugin(singleplayer::SinglePlayerScene)
        .add_state(GameState::MainMenu)
        .run();
}

