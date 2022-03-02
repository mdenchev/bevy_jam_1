use std::{
    mem::replace,
    ops::{Deref, DerefMut},
};

use bevy::prelude::*;
use heron::{CollisionEvent, CollisionLayers, CollisionShape, RigidBody};

use crate::{gun::GunType, inputs::PlayerInput, player::ControlledPlayer, GameLayers, GameState};

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_inventory_ui)
                .with_system(spawn_pickup),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(update_inventory_ui)
                .with_system(drop_pickup)
                .with_system(collide_pickups),
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(despawn_inventory_ui));
    }
}

#[derive(Clone, Component, Debug)]
pub enum Item {
    Gun(GunType),
    Grenade,
    Totem,
}

impl Item {
    fn image_path(&self) -> &'static str {
        match self {
            Item::Gun(gun_type) => match gun_type {
                GunType::Shotgun => "images/shotgun.png",
            },
            Item::Grenade => "images/grenade.png",
            Item::Totem => "images/totem.png",
        }
    }

    fn bundle(self, tf: Transform, asset_server: &AssetServer) -> PickupBundle {
        PickupBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load(self.image_path()),
                transform: tf,
                ..SpriteBundle::default()
            },
            rb: RigidBody::Sensor,
            coll_shape: CollisionShape::Sphere { radius: 10. },
            coll_layers: CollisionLayers::none()
                .with_group(GameLayers::Pickups)
                .with_mask(GameLayers::Player),
            item: self,
        }
    }
}

#[derive(Bundle)]
struct PickupBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    rb: RigidBody,
    coll_shape: CollisionShape,
    coll_layers: CollisionLayers,
    item: Item,
}

#[derive(Component, Default)]
pub struct IgnoreColliders(Vec<Entity>);

impl Deref for IgnoreColliders {
    type Target = Vec<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IgnoreColliders {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component, Debug, Default)]
pub struct Inventory(Option<Item>);

impl Deref for Inventory {
    type Target = Option<Item>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Inventory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Inventory {
    #[allow(dead_code)]
    pub fn collect_item(&mut self, item: Item) -> bool {
        if self.is_none() {
            **self = Some(item);
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn drop_item(&mut self) -> Option<Item> {
        replace(&mut self.0, None)
    }

    pub fn get_item(&self) -> Option<&Item> {
        self.as_ref()
    }
}

#[derive(Component)]
struct InventoryUi;

#[derive(Component)]
struct InventoryUiImage;

#[derive(Component)]
struct UiCamera;

fn spawn_inventory_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Spawning inventory UI");

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::SpaceBetween,
                ..Style::default()
            },
            color: Color::NONE.into(),
            ..NodeBundle::default()
        })
        .insert(InventoryUi)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::FlexEnd,
                        ..Style::default()
                    },
                    color: Color::NONE.into(),
                    ..NodeBundle::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Px(128.), Val::Auto),
                                ..Style::default()
                            },
                            image: asset_server.load("images/empty.png").into(),
                            ..ImageBundle::default()
                        })
                        .insert(InventoryUiImage);
                });
        });
}

fn spawn_pickup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(Item::Grenade.bundle(Transform::from_xyz(300., 300., 1.1), &asset_server));
}

fn update_inventory_ui(
    asset_server: Res<AssetServer>,
    curr_players: Query<&Inventory, (With<ControlledPlayer>, Changed<Inventory>)>,
    mut ui_images: Query<&mut UiImage, With<InventoryUiImage>>,
) {
    if let (Ok(inventory), Ok(mut image)) = (curr_players.get_single(), ui_images.get_single_mut())
    {
        info!("Updating UI for: {inventory:?}");

        *image = asset_server
            .load(
                inventory
                    .get_item()
                    .map_or("images/empty.png", |item| item.image_path()),
            )
            .into();
    }
}

fn drop_pickup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<PlayerInput>,
    mut curr_players: Query<
        (&mut Inventory, &mut IgnoreColliders, &Transform),
        With<ControlledPlayer>,
    >,
) {
    if input.throw.was_pressed() {
        if let Ok((mut inventory, mut ignore_colls, tf)) = curr_players.get_single_mut() {
            if let Some(item) = inventory.drop_item() {
                ignore_colls.push(commands.spawn_bundle(item.bundle(*tf, &asset_server)).id());
            }
        }
    }
}

fn collide_pickups(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    pickups: Query<&Item>,
    mut players: Query<
        (&mut Inventory, &mut IgnoreColliders),
        (With<ControlledPlayer>, Without<Item>),
    >,
) {
    for ev in events.iter() {
        let (e1, e2) = ev.rigid_body_entities();
        let (layer_1, layer_2) = ev.collision_layers();

        let player;
        let pickup;
        if layer_1.contains_group(GameLayers::Pickups) {
            player = e2;
            pickup = e1;
        } else if layer_2.contains_group(GameLayers::Pickups) {
            player = e1;
            pickup = e2;
        } else {
            continue;
        }

        if let (Ok((mut inventory, mut ignore_colls)), Ok(item)) =
            (players.get_mut(player), pickups.get(pickup))
        {
            if ev.is_started() {
                if !ignore_colls.contains(&pickup) && inventory.collect_item(item.clone()) {
                    commands.entity(pickup).despawn();
                }
            } else {
                ignore_colls.retain(|id| *id != pickup);
            }
        }
    }
}

fn despawn_inventory_ui(
    mut commands: Commands,
    uis: Query<Entity, Or<(With<InventoryUi>, With<UiCamera>)>>,
) {
    info!("Despawning inventory UI");

    uis.iter().for_each(|ui| {
        commands.entity(ui).despawn_recursive();
    });
}
