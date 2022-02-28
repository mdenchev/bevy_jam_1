use bevy::prelude::*;

use crate::player::ControlledPlayer;

pub enum Item {
    Gun,
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
        self.held_item = (self.held_item + 1) % self.items.len();
    }

    pub fn hold_prev_item(&mut self) {
        self.held_item = (self.held_item + self.items.len() - 1) % self.items.len();
    }
}

pub fn inventory_ui(mut commands: Commands, players: Query<&Inventory, With<ControlledPlayer>>) {
    if let Some(inventory) = players.get_single() {}
}
