use bevy::prelude::*;
use bevy_rapier2d::{prelude::*};

use crate::inputs::PlayerInput;

use super::PlayerStats;

#[derive(Component, Default)]
pub struct ControllablePlayer;

pub fn move_player(
    player_input: Res<PlayerInput>,
    mut controllable_player: Query<
        (
            &mut RigidBodyVelocityComponent,
            &PlayerStats,
        ),
        With<ControllablePlayer>,
    >,
) {
    for (mut vel, stat) in controllable_player.iter_mut() {
        let new_vel = player_input.move_direction * stat.speed;
        vel.linvel = new_vel.into();
        // vel.apply_impulse(rb_mprops, (new_vel * 40.0).into());
    }
}
