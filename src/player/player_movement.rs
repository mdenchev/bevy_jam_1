use bevy::prelude::*;
use heron::{CollisionLayers, RigidBody, Velocity};

use crate::{
    gun::{GunTimer, GunType},
    inputs::PlayerInput,
    item::{Inventory, Item},
    resources::audio_channels::AudioChannels,
    GameState,
};

use super::{ControlledPlayer, PlayerRecording, PlayerStats};

#[derive(Component, Default)]
pub struct ControllablePlayer;

#[derive(Component, Default)]
pub struct CloneId(pub usize);

pub struct PlayerInputTick {
    pub entity: Entity,
    pub input: PlayerInput,
}

pub fn record_player(
    player_input: Res<PlayerInput>,
    mut player_recording: ResMut<PlayerRecording>,
) {
    let loop_idx = player_recording.current_loop;
    // FIXME this can be done better elsewhere but eh
    if player_recording.inputs.len() <= loop_idx {
        player_recording.inputs.push(vec![]);
    }
    player_recording.inputs[loop_idx].push(player_input.clone());
}

pub fn replay_recordings(
    mut player_recording: ResMut<PlayerRecording>,
    mut input_ticks: EventWriter<PlayerInputTick>,
    mut clones: Query<
        (Entity, &mut Velocity, &PlayerStats, &CloneId),
        (Without<ControlledPlayer>, With<RigidBody>),
    >,
) {
    let current_loop = player_recording.current_loop;
    if current_loop == 0 {
        return;
    };
    let mut ticks_batch = vec![];
    let tick = player_recording.current_tick;
        for (id, recording) in player_recording.inputs[..current_loop].iter().enumerate() {
        for (entity, mut vel, stat, clone_id) in clones.iter_mut() {
            if clone_id.0 == id {
                if let Some(input) = recording.get(tick) {
                    // Movement
                    vel.linear = Vec3::from((input.move_direction, 0.0)) * stat.speed;
                    // Shooting
                    ticks_batch.push(PlayerInputTick {
                        entity,
                        input: input.clone(),
                    })
                } else if let Some(input) = recording.last() {
                    // Movement
                    vel.linear = Vec3::from((input.move_direction, 0.0)) * stat.speed;
                    // Shooting
                    ticks_batch.push(PlayerInputTick {
                        entity,
                        input: input.clone(),
                    })
                }
            }
        }
    }
    input_ticks.send_batch(ticks_batch.into_iter());
    player_recording.current_tick += 1;
}

pub fn player_clone(
    mut keys: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
    mut player_recording: ResMut<PlayerRecording>,
) {
    if keys.just_pressed(KeyCode::C) {
        info!("Cloning!");
        player_recording.current_loop += 1;
        player_recording.current_tick = 0;
        player_recording.inputs.push(vec![]);
        let _ = game_state.overwrite_set(GameState::SetupLevel);
        keys.clear_just_pressed(KeyCode::C);
    }
}

pub fn player_movement(
    player_input: Res<PlayerInput>,
    mut controllable_player: Query<
        (&mut Velocity, &PlayerStats),
        (With<ControlledPlayer>, With<RigidBody>),
    >,
) {
    for (mut vel, stat) in controllable_player.iter_mut() {
        vel.linear = Vec3::from((player_input.move_direction, 0.0)) * stat.speed;
    }
}

pub fn player_shooting_input(
    player_input: Res<PlayerInput>,
    mut input_ticks: EventWriter<PlayerInputTick>,
    players: Query<Entity, With<ControlledPlayer>>,
) {
    if let Ok(entity) = players.get_single() {
        input_ticks.send(PlayerInputTick {
            entity,
            input: player_input.clone(),
        })
    }
}

pub fn player_shooting(
    mut commands: Commands,
    audio: Res<bevy_kira_audio::Audio>,
    channels: Res<AudioChannels>,
    asset_server: Res<AssetServer>,
    mut input_ticks: EventReader<PlayerInputTick>,
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
    for PlayerInputTick { input, entity } in input_ticks.iter() {
        let entity = *entity;
        let Ok((player_ent, &player_transform, inventory)) = players.get(entity) else {return};
        for (parent, mut gun_transform, mut visibility, mut gun_timer, gun_type) in guns.iter_mut()
        {
            if let Some(Item::Gun(_)) = inventory.get_item() {
                visibility.is_visible = true;

                if parent.0 == player_ent {
                    gun_timer.tick(time.delta());
                    // Shoot
                    if input.shoot.is_down() && gun_timer.finished() {
                        info!("Player {player_ent:?} shoots {gun_type:?}");
                        gun_type.play_sfx(&*audio, &channels.audio, &*asset_server);
                        commands
                            .spawn_bundle(gun_type.create_bullet_bundle(
                                &*asset_server,
                                player_transform.translation + gun_transform.translation,
                                input.aim_direction,
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
                        input.aim_direction.y.atan2(input.aim_direction.x),
                    );
                }
            } else {
                visibility.is_visible = false;

                gun_timer.set_duration(gun_type.cooldown());
                gun_timer.reset();
            }
        }
    }
}
