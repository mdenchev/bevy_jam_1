use bevy::prelude::*;
use heron::{CollisionLayers, RigidBody, Velocity};

use crate::{
    gun::{GunTimer, GunType},
    inputs::PlayerInput,
    item::{Inventory, Item},
    resources::audio_channels::AudioChannels,
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
    mut commands: Commands,
    audio: Res<bevy_kira_audio::Audio>,
    channels: Res<AudioChannels>,
    asset_server: Res<AssetServer>,
    player_input: Res<PlayerInput>,
    time: Res<Time>,
    players: Query<(Entity, &Transform, &Inventory), With<ControllablePlayer>>,
    mut guns: Query<
        (
            &Parent,
            &mut Transform,
            &mut Visibility,
            &mut GunTimer,
            &GunType,
        ),
        Without<ControllablePlayer>,
    >,
) {
    let Ok((player_ent, &player_transform, inventory)) = players.get_single() else {return};
    for (parent, mut gun_transform, mut visibility, mut gun_timer, gun_type) in guns.iter_mut() {
        if let Some(Item::Gun(_)) = inventory.get_item() {
            visibility.is_visible = true;

            if parent.0 == player_ent {
                gun_timer.tick(time.delta());
                // Shoot
                if player_input.shoot.is_down() && gun_timer.finished() {
                    info!("Player shoots {gun_type:?}");
                    gun_type.play_sfx(&*audio, &channels.audio, &*asset_server);
                    commands
                        .spawn_bundle(gun_type.create_bullet_bundle(
                            &*asset_server,
                            player_transform.translation + gun_transform.translation,
                            player_input.aim_direction,
                        ))
                        .insert(
                            CollisionLayers::none()
                                .with_group(crate::GameLayers::Bullets)
                                .with_masks(&[
                                    crate::GameLayers::World,
                                    crate::GameLayers::Enemies,
                                ]),
                        );
                    gun_timer.set_duration(gun_type.cooldown());
                    gun_timer.reset();
                }
                // Orient gun
                gun_transform.rotation = Quat::from_axis_angle(
                    Vec3::Z,
                    player_input
                        .aim_direction
                        .y
                        .atan2(player_input.aim_direction.x),
                );
            }
        } else {
            visibility.is_visible = false;

            gun_timer.set_duration(gun_type.cooldown());
            gun_timer.reset();
        }
    }
}
