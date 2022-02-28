use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use bevy::{core::FixedTimestep, prelude::*};

mod player_movement;

use heron::{CollisionShape, RigidBody, RotationConstraints, Velocity};
use player_movement::handle_player_input;

use crate::{utils::CommonHandles, GameState};

use self::player_movement::ControllablePlayer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            //PlayerStage,
            SystemSet::on_enter(GameState::Playing).with_system(spawn_player),
        )
        .add_system(
            //PlayerStage,
            handle_player_input.with_run_criteria(FixedTimestep::steps_per_second(60.0)),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(cam_follow_player));
    }
}

// Going to want this to find the spawn point eventually.
fn spawn_player(mut commands: Commands, common_handles: Res<CommonHandles>) {
    info!("Spawning player!");
    commands
        .spawn_bundle(ControllablePlayerBundle::default())
        .insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(32),
            texture_atlas: common_handles.player_sprites.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        .insert(CollisionShape::Sphere { radius: 10.0 })
        .insert(Velocity::default());
}

#[derive(Bundle, Default)]
pub struct ControllablePlayerBundle {
    controllable: ControllablePlayer,
    stats: PlayerStats,
    shoot_timer: ShootTimer,
}

#[derive(Component)]
pub struct ShootTimer(Timer);

impl Deref for ShootTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ShootTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for ShootTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(1), true))
    }
}

#[derive(Component)]
pub struct PlayerStats {
    pub speed: f32,
    pub shoot_cooldown: Duration,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            speed: 200.0,
            shoot_cooldown: Duration::from_millis(400),
        }
    }
}

fn cam_follow_player(
    mut queries: QuerySet<(
        QueryState<&mut Transform, With<Camera>>,
        QueryState<&Transform, (With<ControllablePlayer>, With<RigidBody>)>,
    )>,
) {
    let mut player_position = if let Ok(player) = queries.q1().get_single() {
        player.translation
    } else {
        return;
    };
    if let Ok(mut camera) = queries.q0().get_single_mut() {
        player_position.z = camera.translation.z;
        camera.translation = player_position;
    }
}
