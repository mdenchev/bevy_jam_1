use bevy::prelude::*;
use bevy::window::WindowMode;
use heron::prelude::*;
use inputs::GameInputPlugin;

mod player;
mod inputs;

fn main() {
    App::new()
        // Configure the game window
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 900.0,
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
        .run();
}
