use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use heron::{CollisionShape, RigidBody};

use crate::{player::PlayerStats, GameState};

#[derive(Copy, Clone, Debug)]
pub struct NewPos(f32, f32);

pub struct SinglePlayerScene;

impl Plugin for SinglePlayerScene {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_system(crate::utils::set_texture_filters_to_nearest)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(level_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(zoom_update)
                    .with_system(set_player_pos),
            );
    }
}

fn level_setup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    info!("[Scene:SingleplayerLevel:setup]");
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
        layer_builder.for_each_tiles_mut(|_ent, data| {
            *data = Some(if rand::random::<f32>() > 0.55 {
                floor_tile.clone()
            } else {
                outside_tile.clone()
            });
        });

        let (size_x, size_y) = {
            let settings = &layer_builder.settings;
            (
                settings.map_size.0 * settings.chunk_size.0,
                settings.map_size.1 * settings.chunk_size.1,
            )
        };

        for y in 0..size_y {
            for x in [0, size_x - 1] {
                *layer_builder.get_tile_mut(TilePos(x, y)).unwrap() = outside_tile.clone();
            }
        }
        for x in 0..size_x {
            for y in [0, size_y - 1] {
                *layer_builder.get_tile_mut(TilePos(x, y)).unwrap() = outside_tile.clone();
            }
        }

        for _ in 0..40 {
            let mut new_vals = vec![];
            for y in 1..(size_y - 1) {
                for x in 1..(size_x - 1) {
                    let mut neighbors = 0;
                    for yp in [y - 1, y, y + 1] {
                        for xp in [x - 1, x, x + 1] {
                            if xp == x && yp == y {
                                continue;
                            }
                            if let Ok(tile) = layer_builder
                                .get_tile(TilePos(xp, yp))
                                .map(|t| t.tile.texture_index)
                            {
                                if tile == 9 {
                                    neighbors += 1;
                                }
                            }
                        }
                    }
                    new_vals.push((x, y, neighbors > 4 || neighbors == 0));
                }
            }
            for (x, y, should_be_wall) in new_vals {
                layer_builder
                    .get_tile_mut(TilePos(x, y))
                    .unwrap()
                    .tile
                    .texture_index = if should_be_wall { 9 } else { 4 };
            }
        }

        let mut player_transform: Option<NewPos> = None;

        // Get rid of hanging pockets, add the
        for y in 0..size_y {
            for x in 0..size_x {
                let tile_pos = TilePos(x, y);
                let (x_px, y_px) = (
                    tile_pos.0 as f32 * 32.0 + 16.0,
                    tile_pos.1 as f32 * 32.0 + 16.0,
                );
                if layer_builder.get_tile(tile_pos).unwrap().tile.texture_index != 9 {
                    player_transform = Some(player_transform.unwrap_or(NewPos(x_px, y_px)));
                    continue;
                }
                let mut neighbors = 0;
                if x > 0 && x < size_x && y > 0 && y < size_y {
                    for yp in [y - 1, y, y + 1] {
                        for xp in [x - 1, x, x + 1] {
                            if (xp == x && yp == y) || !(xp == x || yp == y) {
                                continue;
                            }
                            if let Ok(tile) = layer_builder
                                .get_tile(TilePos(xp, yp))
                                .map(|t| t.tile.texture_index)
                            {
                                if tile == 4 {
                                    neighbors += 1;
                                }
                            }
                        }
                    }
                }
                if neighbors == 4 {
                    *layer_builder.get_tile_mut(tile_pos).unwrap() = floor_tile.clone();
                } else {
                    let child = commands
                        .spawn()
                        .insert(GlobalTransform::from_xyz(x_px, y_px, 0.0))
                        .insert(RigidBody::Static)
                        .insert(CollisionShape::Cuboid {
                            half_extends: Vec3::new(16.0, 16.0, 0.0),
                            border_radius: None,
                        })
                        .id();
                    commands.entity(map_entity).add_child(child);
                }
            }
        }

        println!("{player_transform:?}");
        commands.insert_resource(player_transform.unwrap());
    }

    let layer_entity = map_query.build_layer(&mut commands, layer_builder, texture_handle);

    map.add_layer(&mut commands, 0u16, layer_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
        .insert(GlobalTransform::default());
}

fn set_player_pos(
    mut query: Query<&mut Transform, With<PlayerStats>>,
    pos: Option<Res<NewPos>>,
    mut commands: Commands,
) {
    for mut t in query.iter_mut() {
        if let Some(ref p) = pos {
            println!("{p:?}");
            *t = Transform::from_xyz(p.0, p.1, 10.0);
            commands.remove_resource::<NewPos>();
        }
    }
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
