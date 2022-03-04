pub mod map;

use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody, Velocity};

use crate::{player::PlayerRecording, utils::CommonHandles, GameState};

use self::map::MapInitData;

pub struct SinglePlayerScene;

impl Plugin for SinglePlayerScene {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .init_resource::<MapInitData>()
            .add_system(crate::utils::set_texture_filters_to_nearest)
            .add_system_set(SystemSet::on_enter(GameState::BuildLevel).with_system(build_level))
            .add_system_set(SystemSet::on_enter(GameState::SetupLevel).with_system(level_spawns))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(zoom_update)
                    .with_system(update_kills_text),
            )
            .add_system_set(SystemSet::on_enter(GameState::GameWon).with_system(game_won));
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct KilledText;

fn build_level(
    mut commands: Commands,
    common_handles: Res<CommonHandles>,
    mut game_state: ResMut<State<GameState>>,
    mut map_init_data: ResMut<MapInitData>,
    asset_server: Res<AssetServer>,
    atlases: Res<Assets<TextureAtlas>>,
    mut map_query: MapQuery,
) {
    info!("[Scene:SingleplayerLevel:setup]");
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    let texture_handle = atlases
        .get(common_handles.player_sprites.clone())
        .unwrap()
        .texture
        .clone();

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

        // Get rid of hanging pockets, add the
        for y in 0..size_y {
            for x in 0..size_x {
                let tile_pos = TilePos(x, y);
                let (x_px, y_px) = (
                    tile_pos.0 as f32 * 32.0 + 16.0,
                    tile_pos.1 as f32 * 32.0 + 16.0,
                );
                if layer_builder.get_tile(tile_pos).unwrap().tile.texture_index != 9 {
                    map_init_data.player_spawn_position = (x_px, y_px);
                    if rand::random::<f32>() < 0.1 {
                        map_init_data.enemy_spawn_positions.push((x_px, y_px));
                    }
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
                        .insert(
                            CollisionLayers::none()
                                .with_group(crate::GameLayers::World)
                                .with_masks(&[
                                    crate::GameLayers::Player,
                                    crate::GameLayers::Bullets,
                                    crate::GameLayers::Enemies,
                                ]),
                        )
                        .id();
                    commands.entity(map_entity).add_child(child);
                }
            }
        }
    }

    let layer_entity = map_query.build_layer(&mut commands, layer_builder, texture_handle);

    map.add_layer(&mut commands, 0u16, layer_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
        .insert(GlobalTransform::default());

    // Fixme bad place for this but no time to be clean
    // Ui stuff
    let text_style = Style {
        align_self: AlignSelf::Center,
        position_type: PositionType::Relative,
        position: Rect::default(),
        ..Default::default()
    };

    let text_textstyle = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::BLACK,
    };

    let text_text_alignment = TextAlignment {
        horizontal: HorizontalAlign::Center,
        vertical: VerticalAlign::Center,
    };

    let root_ui_ent = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(40.),
                    top: Val::Px(50.),
                    ..Default::default()
                },
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let goal_ent = commands
        .spawn_bundle(TextBundle {
            style: text_style.clone(),
            text: Text::with_section(
                "Kill 50 enemies as fast as possible!",
                text_textstyle.clone(),
                text_text_alignment,
            ),
            ..Default::default()
        })
        .id();
    let kills_ent = commands
        .spawn_bundle(TextBundle {
            style: text_style.clone(),
            text: Text::with_section(
                format!("Kills: {}", map_init_data.kills),
                text_textstyle.clone(),
                text_text_alignment,
            ),
            ..Default::default()
        })
        .insert(KilledText)
        .id();
    commands
        .entity(root_ui_ent)
        .push_children(&[goal_ent, kills_ent]);

    let _ = game_state.overwrite_set(GameState::SetupLevel);
}

fn zoom_update(
    mut scroll: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    for mut projection in query.iter_mut() {
        for ev in scroll.iter() {
            projection.scale = (projection.scale - ev.y / 20.0).max(0.01);
        }
    }
}

pub fn update_kills_text(
    map_init_data: Res<MapInitData>,
    mut query: Query<&mut Text, With<KilledText>>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("Kills: {}", map_init_data.kills);
}

pub fn level_spawns(
    mut commands: Commands,
    common_handles: Res<CommonHandles>,
    mut game_state: ResMut<State<GameState>>,
    mut map_init_data: ResMut<MapInitData>,
    recordings: Res<PlayerRecording>,
    asset_server: Res<AssetServer>,
    char_query: Query<Entity, (With<Velocity>, With<RigidBody>)>,
) {
    info!("Setting up level ents");
    // Reset kills
    map_init_data.kills = 0;

    // Clear existing enemies
    for ent in char_query.iter() {
        commands.entity(ent).despawn_recursive();
    }

    // Spawn player
    crate::player::spawn_player(
        &mut commands,
        &common_handles,
        map_init_data.player_spawn_position,
        &asset_server,
        false,
        10000, // doesn't matter
    );

    // Spawn clones
    for id in 0..recordings.inputs.len() {
        crate::player::spawn_player(
            &mut commands,
            &common_handles,
            map_init_data.player_spawn_position,
            &asset_server,
            true,
            id,
        );
    }

    // Spawn enemies
    for (x_px, y_px) in map_init_data.enemy_spawn_positions.iter().cloned() {
        crate::enemy::spawn_enemy(&mut commands, &common_handles, Vec2::new(x_px, y_px));
    }

    let _ = game_state.overwrite_set(GameState::Playing);
}

pub fn game_won(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_init_data: ResMut<MapInitData>,
    query: Query<Entity>,
) {
    info!("Game Won!");
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }

    commands.spawn_bundle(UiCameraBundle::default());
    let text_style = Style {
        align_self: AlignSelf::Center,
        position_type: PositionType::Relative,
        position: Rect::default(),
        ..Default::default()
    };

    let text_textstyle = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::BLACK,
    };

    let text_text_alignment = TextAlignment {
        horizontal: HorizontalAlign::Center,
        vertical: VerticalAlign::Center,
    };

    let root_ui_ent = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let win_ent = commands
        .spawn_bundle(TextBundle {
            style: text_style.clone(),
            text: Text::with_section(
                format!(
                    "You did it! In only {} seconds!",
                    map_init_data.timer.as_secs()
                ),
                text_textstyle.clone(),
                text_text_alignment,
            ),
            ..Default::default()
        })
        .id();
    commands.entity(root_ui_ent).push_children(&[win_ent]);
}
