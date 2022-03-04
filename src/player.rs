use bevy::{core::FixedTimestep, prelude::*};

mod player_movement;

use heron::{CollisionLayers, CollisionShape, RigidBody, RotationConstraints, Velocity};
use player_movement::player_movement;

use crate::{
    gun::GunType,
    inputs::PlayerInput,
    item::{IgnoreColliders, Inventory, Item},
    levels::MainCamera,
    player::player_movement::CloneId,
    utils::CommonHandles,
    GameState,
};

use self::player_movement::{
    player_shooting, record_player, replay_recordings, ControllablePlayer, player_clone, PlayerInputTick, player_shooting_input,
};

#[derive(Default)]
pub struct PlayerRecording {
    pub current_loop: usize,
    pub current_tick: usize,
    pub inputs: Vec<Vec<PlayerInput>>,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerRecording>()
            .add_event::<PlayerInputTick>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(cam_follow_player)
                    .with_system(record_player)
                    .with_system(player_clone)
                    .with_system(player_movement)
                    .with_system(replay_recordings)
                    .with_system(player_shooting_input)
                    .with_system(player_shooting)
                    // This apparently removes the GameState condition
                    //.with_run_criteria(
                    //    FixedTimestep::steps_per_second(60.0),
                    //)
            );
    }
}

// Going to want this to find the spawn point eventually.
pub fn spawn_player(
    commands: &mut Commands,
    common_handles: &CommonHandles,
    pos: (f32, f32),
    asset_server: &AssetServer,
    is_clone: bool,
    clone_id: usize,
) {
    if is_clone {
        info!("Spawning clone#{clone_id}");
    } else {
        info!("Spawning player!");
    }
    let starting_gun = commands
        .spawn_bundle(GunType::Shotgun.create_bundle(&*asset_server))
        .id();

    let mut starting_inventory = Inventory::default();
    starting_inventory.collect_item(Item::Gun(GunType::Shotgun));

    let player_ent = commands
        .spawn_bundle(ControllablePlayerBundle::default())
        .insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(32),
            texture_atlas: common_handles.player_sprites.clone(),
            transform: Transform::from_xyz(pos.0, pos.1, 1.0),
            ..Default::default()
        })
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
        .add_child(starting_gun)
        .id();
    if is_clone {
        commands.entity(player_ent).insert(CloneId(clone_id));
    } else {
        commands.entity(player_ent).insert(ControlledPlayer);
    }
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
        QueryState<&Transform, (With<ControlledPlayer>, With<RigidBody>)>,
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