use bevy::prelude::*;

use crate::GameState;

pub mod common;
pub mod main_menu;

pub struct MainMenuScene;

impl Plugin for MainMenuScene {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(main_menu::setup))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu).with_system(main_menu::handle_buttons),
            )
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(common::cleanup));
    }
}
