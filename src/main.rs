use bevy::prelude::*;
use bevy::window::WindowMode;
use template_lib::*;

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
        // Add plugins here
        .add_plugin(HelloWorldPlugin)
        .run();
}
