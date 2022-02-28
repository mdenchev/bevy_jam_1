use bevy::{core::FixedTimestep, prelude::*};
use bevy_rapier2d::prelude::*;

mod player_movement;

use player_movement::move_player;

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
            move_player.with_run_criteria(FixedTimestep::steps_per_second(60.0)),
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
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::ball(14.0).into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete);
}

#[derive(Bundle, Default)]
pub struct ControllablePlayerBundle {
    controllable: ControllablePlayer,
    stats: PlayerStats,
}

#[derive(Component)]
pub struct PlayerStats {
    pub speed: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self { speed: 200.0 }
    }
}

fn cam_follow_player(
    mut queries: QuerySet<(
        QueryState<&mut Transform, With<Camera>>,
        QueryState<&Transform, (With<ControllablePlayer>, With<RigidBodyPositionComponent>)>,
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
