use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use rand::prelude::Distribution;

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
    floor_tile.tile.texture_index = 4;

    {
        use rand::distributions::Uniform;
        let mut rng = rand::thread_rng();
        let size_distrib = Uniform::new(5, 15);
        let settings = layer_builder.settings.clone();
        for _ in 0..Uniform::new(10, 37).sample(&mut rng) {
            let size_x = size_distrib.sample(&mut rng);
            let size_y = size_distrib.sample(&mut rng);
            let x = Uniform::new(1, settings.map_size.0 * settings.chunk_size.0 - size_x)
                .sample(&mut rng);
            let y = Uniform::new(1, settings.map_size.1 * settings.chunk_size.1 - size_y)
                .sample(&mut rng);
            layer_builder.fill(
                TilePos(x, y),
                TilePos(x + size_x, y + size_y),
                floor_tile.clone(),
            );
        }
    }
    layer_builder.for_each_tiles_mut(|tile_entity, tile_data| {
        if tile_data.is_none() {
            *tile_data = Some(outside_tile.clone());
        }

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
            projection.scale = (projection.scale - ev.y / 20.0).max(0.01);
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
