use std::{
    mem::replace,
    ops::{Deref, DerefMut},
};

use bevy::prelude::*;

use crate::{gun::GunType, player::ControlledPlayer, GameState};

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_inventory_ui))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(update_inventory_ui),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(despawn_inventory_ui),
            );
    }
}

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
}

#[derive(Component, Default)]
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

fn update_inventory_ui(
    asset_server: Res<AssetServer>,
    curr_players: Query<&Inventory, (With<ControlledPlayer>, Changed<Inventory>)>,
    mut ui_images: Query<&mut UiImage, With<InventoryUiImage>>,
) {
    if let (Ok(inventory), Ok(mut image)) = (curr_players.get_single(), ui_images.get_single_mut())
    {
        *image = asset_server
            .load(
                inventory
                    .get_item()
                    .map_or("images/empty.png", |item| item.image_path()),
            )
            .into();
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
