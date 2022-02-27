use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_ecs_tilemap::prelude::*;

use crate::GameState;

pub struct SinglePlayerScene;

impl Plugin for SinglePlayerScene {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_system(crate::utils::set_texture_filters_to_nearest)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(level_setup))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(zoom_update));
    }
}

fn level_setup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let texture_handle = asset_server.load("images/images.png");

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            MapSize(2, 2),
            ChunkSize(64, 64),
            TileSize(32.0, 32.0),
            TextureSize(96.0, 128.0),
        ),
        0u16,
        0u16,
    );

    let mut outside_tile = TileBundle::default();
    outside_tile.tile.texture_index = 9;
    let mut floor_tile = TileBundle::default();
    floor_tile.tile.texture_index = 0;

    layer_builder.for_each_tiles_mut(|tile_entity, tile_data| {
        *tile_data = Some(if rand::random() {
            outside_tile.clone()
        } else {
            floor_tile.clone()
        });

        if tile_entity.is_none() {
            *tile_entity = Some(commands.spawn().id());
        }
    });

    let layer_entity = map_query.build_layer(&mut commands, layer_builder, texture_handle);

    map.add_layer(&mut commands, 0u16, layer_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
        .insert(GlobalTransform::default());
}

fn zoom_update(
    mut scroll: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for mut projection in query.iter_mut() {
        for ev in scroll.iter() {
            let mut log_scale = projection.scale.ln();
            log_scale += ev.y;
            projection.scale = log_scale.exp();
        }
    }
}

// fn list_tiles(query: Query<)

fn fill_layer<T: TileBundleTrait + Clone>(
    lb: &mut LayerBuilder<T>,
    start: TilePos,
    end: TilePos,
    tile: T,
) {
    for x in start.0..end.0 {
        for y in start.1..end.1 {
            lb.set_tile(TilePos(x, y), tile.clone()).unwrap();
        }
    }
}
