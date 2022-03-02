use bevy::{core::FixedTimestep, prelude::*};

mod player_movement;

use heron::{CollisionLayers, CollisionShape, RigidBody, RotationConstraints, Velocity};
use player_movement::player_movement;

use crate::{
    gun::GunType,
    item::{IgnoreColliders, Inventory, Item},
    levels::MainCamera,
    utils::CommonHandles,
    GameState,
};

use self::player_movement::{player_shooting, ControllablePlayer};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            //PlayerStage,
            SystemSet::on_enter(GameState::Playing).with_system(spawn_player),
        )
        .add_system(
            //PlayerStage,
            player_movement.with_run_criteria(FixedTimestep::steps_per_second(60.0)),
        )
        .add_system(
            //PlayerStage,
            player_shooting.with_run_criteria(FixedTimestep::steps_per_second(60.0)),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(cam_follow_player));
    }
}

// Going to want this to find the spawn point eventually.
fn spawn_player(
    mut commands: Commands,
    common_handles: Res<CommonHandles>,
    asset_server: Res<AssetServer>,
) {
    info!("Spawning player!");
    let starting_gun = commands
        .spawn_bundle(GunType::Shotgun.create_bundle(&*asset_server))
        .id();

    let mut starting_inventory = Inventory::default();
    starting_inventory.collect_item(Item::Gun(GunType::Shotgun));

    commands
        .spawn_bundle(ControllablePlayerBundle::default())
        .insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(32),
            texture_atlas: common_handles.player_sprites.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(ControlledPlayer)
        .insert(starting_inventory)
        .insert(IgnoreColliders::default())
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        .insert(CollisionShape::Sphere { radius: 10.0 })
        .insert(Velocity::default())
        .insert(
            CollisionLayers::none()
                .with_group(crate::GameLayers::Player)
                .with_masks(&[
                    crate::GameLayers::World,
                    crate::GameLayers::Enemies,
                    crate::GameLayers::Pickups,
                ]),
        )
        .add_child(starting_gun);
}

#[derive(Bundle, Default)]
pub struct ControllablePlayerBundle {
    controllable: ControllablePlayer,
    stats: PlayerStats,
    inventory: Inventory,
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

// Should mark the player currently under control but not ghosts
#[derive(Component)]
pub struct ControlledPlayer;

fn cam_follow_player(
    mut queries: QuerySet<(
        QueryState<&mut Transform, With<MainCamera>>,
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
