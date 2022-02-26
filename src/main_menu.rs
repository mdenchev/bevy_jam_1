use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::GameState;

pub struct MainMenuScene;

impl Plugin for MainMenuScene {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::MainMenu).add_system_set(
            SystemSet::on_update(GameState::MainMenu).with_system(main_menu_system),
        );
    }
}

fn main_menu_system(mut egui_context: ResMut<EguiContext>, mut state: ResMut<State<GameState>>) {
    egui::Window::new("Bevy game").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Singleplayer").clicked() {
            state.overwrite_set(GameState::Playing).unwrap();
        }
    });
}
