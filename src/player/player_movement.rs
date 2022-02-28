use bevy::prelude::*;
use heron::{RigidBody, Velocity};

use crate::inputs::PlayerInput;

use super::{PlayerStats, ShootTimer};

#[derive(Component, Default)]
pub struct ControllablePlayer;

pub fn handle_player_input(
    player_input: Res<PlayerInput>,
    time: Res<Time>,
    mut controllable_player: Query<
        (&mut Velocity, &PlayerStats, &mut ShootTimer),
        (With<ControllablePlayer>, With<RigidBody>),
    >,
) {
    for (mut vel, stat, mut shoot_timer) in controllable_player.iter_mut() {
        // Velocity
        vel.linear = Vec3::from((player_input.move_direction, 0.0)) * stat.speed;

        // Shooting
        shoot_timer.tick(time.delta());
        if player_input.shoot.is_down() && shoot_timer.finished() {
            info!("Player shot");
            dbg!(&stat.shoot_cooldown);
            shoot_timer.set_duration(stat.shoot_cooldown);
        }
    }
}
