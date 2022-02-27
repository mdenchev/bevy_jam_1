#![feature(try_blocks)]
#![feature(let_else)]

use bevy::prelude::*;
use bevy::window::WindowMode;
use heron::prelude::*;
use inputs::GameInputPlugin;
use bevy_egui::EguiPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod inputs;
mod player;
mod menus;
mod levels;
mod utils;

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
        // Standard Bevy functionality
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(player::PlayerPlugin)
        .add_plugin(inputs::GameInputPlugin)
        // Add plugins here
        .insert_resource(ClearColor(Color::rgb(0.11, 0.039, 0.004)))
        // Standard Bevy functionality
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(EguiPlugin)
        .add_plugin(menus::MainMenuScene)
        .add_plugin(levels::SinglePlayerScene)
        .add_state(GameState::MainMenu)
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    MainMenu,
    Playing,
    GameOver,
}