use std::ops::Deref;

use bevy::prelude::*;

use crate::{gun::GunType, inputs::PlayerInput, player::ControlledPlayer, GameState};

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_inventory_ui))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(scroll_inventory)
                    .with_system(update_inventory_ui),
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
pub struct Inventory {
    items: Vec<Item>,
    held_item: usize,
}

impl Inventory {
    pub fn add_item(&mut self, item: Item) {
        if self.held_item > self.items.len() {
            self.held_item = self.items.len();
        }
        self.items.push(item);
    }

    pub fn drop_item(&mut self) -> Option<Item> {
        if self.held_item < self.items.len() {
            Some(self.items.remove(self.held_item))
        } else {
            None
        }
    }

    pub fn get_held_item(&self) -> Option<&Item> {
        self.items.get(self.held_item)
    }

    pub fn hold_next_item(&mut self) {
        if !self.items.is_empty() {
            self.held_item = (self.held_item + 1) % self.items.len();
        }
    }

    pub fn hold_prev_item(&mut self) {
        if !self.items.is_empty() {
            self.held_item = (self.held_item + self.items.len() - 1) % self.items.len();
        }
    }

    pub fn get_next_item(&self) -> Option<&Item> {
        if self.items.is_empty() {
            None
        } else {
            self.items.get((self.held_item + 1) % self.items.len())
        }
    }

    pub fn get_prev_item(&self) -> Option<&Item> {
        if self.items.is_empty() {
            None
        } else {
            self.items
                .get((self.held_item + self.items.len() - 1) % self.items.len())
        }
    }
}

#[derive(Component)]
struct InventoryUi;

struct InventoryUiSlots([Entity; 3]);

impl Deref for InventoryUiSlots {
    type Target = [Entity; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Component)]
struct UiCamera;

fn spawn_inventory_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Spawning inventory UI");

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);

    let mut slots = [None, None, None];

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
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..Style::default()
                    },
                    color: Color::NONE.into(),
                    ..NodeBundle::default()
                })
                .with_children(|parent| {
                    (0..=2).for_each(|slot| {
                        slots[slot] = Some(
                            parent
                                .spawn_bundle(ImageBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(96.), Val::Auto),
                                        ..Style::default()
                                    },
                                    image: asset_server.load("images/empty.png").into(),
                                    color: Color::rgba(
                                        1.,
                                        1.,
                                        1.,
                                        if slot == 1 { 1. } else { 0.5 },
                                    )
                                    .into(),
                                    ..ImageBundle::default()
                                })
                                .id(),
                        );
                    })
                });
        });

    // Can't .map() iter to array :(
    let err = "An inventory slot was not created";
    commands.insert_resource(InventoryUiSlots([
        slots[0].expect(err),
        slots[1].expect(err),
        slots[2].expect(err),
    ]));
}

fn scroll_inventory(
    mut curr_players: Query<&mut Inventory, With<ControlledPlayer>>,
    player_input: Res<PlayerInput>,
) {
    curr_players.for_each_mut(|mut inventory| {
        if player_input.inventory_next.was_pressed() {
            info!("test");
            inventory.hold_next_item();
        }
        if player_input.inventory_prev.was_pressed() {
            info!("test");
            inventory.hold_prev_item();
        }
    });
}

fn update_inventory_ui(
    asset_server: Res<AssetServer>,
    curr_players: Query<&Inventory, (With<ControlledPlayer>, Changed<Inventory>)>,
    mut ui_images: Query<&mut UiImage>,
    slots: Res<InventoryUiSlots>,
) {
    if let Ok(inventory) = curr_players.get_single() {
        (0..=2).for_each(|slot| {
            *ui_images
                .get_mut(slots[slot])
                .expect("Dangling Entity referring to an inventory slot") = asset_server
                .load(
                    match slot {
                        0 => inventory.get_prev_item(),
                        1 => inventory.get_held_item(),
                        2 => inventory.get_next_item(),
                        _ => unreachable!(),
                    }
                    .map_or("images/empty.png", |item| item.image_path()),
                )
                .into();
        });
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
