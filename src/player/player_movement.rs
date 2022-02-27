use bevy::prelude::*;
use heron::{RigidBody, Velocity};

use crate::inputs::PlayerInput;

use super::PlayerStats;

#[derive(Component, Default)]
pub struct ControllablePlayer;

pub fn move_player(
    player_input: Res<PlayerInput>,
    mut controllable_player: Query<
        (&mut Velocity, &PlayerStats),
        (With<ControllablePlayer>, With<RigidBody>),
    >,
) {
    for (mut vel, stat) in controllable_player.iter_mut() {
        vel.linear = Vec3::from((player_input.move_direction, 0.0)) * stat.speed;
    }
}
