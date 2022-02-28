use bevy::prelude::*;
use heron::{RigidBody, Velocity};

use crate::{
    gun::{GunTimer, GunType},
    inputs::PlayerInput,
};

use super::PlayerStats;

#[derive(Component, Default)]
pub struct ControllablePlayer;

pub fn player_movement(
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

pub fn player_shooting(
    player_input: Res<PlayerInput>,
    time: Res<Time>,
    mut gun_query: Query<(&Parent, &mut GunTimer, &GunType)>,
    player_query: Query<Entity, With<ControllablePlayer>>,
) {
    for (parent, mut gun_timer, gun_type) in gun_query.iter_mut() {
        if player_query.get(parent.0).is_ok() {
            gun_timer.tick(time.delta());
            if player_input.shoot.is_down() && gun_timer.finished() {
                info!("Player shoots {gun_type:?}");
                gun_timer.set_duration(gun_type.cooldown());
            }
        }
    }
}
