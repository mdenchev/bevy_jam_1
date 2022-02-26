use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("images/player.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0, 64.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert_bundle(InputManagerBundle::<Action> {
            // Stores "which virtual action buttons are currently pressed"
            action_state: ActionState::default(),
            // Stores how those actions relate to inputs from your player
            input_map: InputMap::new([
                (Action::MoveLeft, KeyCode::A),
                (Action::MoveRight, KeyCode::D),
            ]),
        });
}

pub fn cleanup_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    commands.entity(query.single()).despawn();
}

/// The player.
#[derive(Debug, Component, Clone, Copy)]
pub struct Player;

/// The possible keyboard button actions a player can do.
#[allow(missing_docs)]
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    MoveLeft,
    MoveRight,
}
