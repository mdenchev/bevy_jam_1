#![feature(try_blocks)]
#![feature(let_else)]
#![allow(clippy::too_many_arguments)]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use heron::prelude::*;
use resources::audio_channels::AudioChannels;

mod enemy;
pub mod gun;
mod inputs;
mod item;
mod levels;
mod menus;
mod player;
pub mod resources;
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
        .insert_resource(ClearColor(Color::rgb(0.11, 0.039, 0.004)))
        .init_resource::<AudioChannels>()
        // Standard Bevy functionality
        .add_plugins(DefaultPlugins)
        .add_plugin(utils::UtilsPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(inputs::GameInputPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(EguiPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(player::PlayerPlugin)
        .add_plugin(menus::MainMenuScene)
        .add_plugin(levels::SinglePlayerScene)
        .add_plugin(item::ItemPlugin)
        .add_plugin(gun::GunPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_state(GameState::MainMenu)
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    MainMenu,
    BuildLevel,
    SetupLevel,
    Playing,
    GameOver,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, PhysicsLayer)]
pub enum GameLayers {
    World,
    Bullets,
    Player,
    Enemies,
    Pickups,
}
