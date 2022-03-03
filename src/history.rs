use bevy::{
    core::FixedTimestep,
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};
use derive_more::{Deref, DerefMut};
use heron::{RigidBody, Velocity};

use crate::GameState;

#[derive(Component, Deref, DerefMut, Default)]
pub struct PositionHistory(Vec<Vec3>);

#[derive(Component, Deref, DerefMut, Default)]
pub struct RewindState(bool);

pub struct RewindPlugin;

impl Plugin for RewindPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(20.0))
                .with_system(add_trackers)
                .with_system(detect_rewind)
                .with_system(tick_position_trackers),
        )
        .insert_resource(RewindState(false))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(60.0))
                .with_system(rewind_system),
        );
    }
}

pub fn add_trackers(
    mut commands: Commands,
    entities: Query<Entity, (With<Velocity>, With<RigidBody>, Without<PositionHistory>)>,
) {
    for entity in entities.iter() {
        commands.entity(entity).insert(PositionHistory::default());
    }
}

pub fn tick_position_trackers(
    mut query: Query<(&mut PositionHistory, &Transform)>,
    rewind_state: Res<RewindState>,
) {
    if rewind_state.0 {
        return;
    }
    for (mut history, transform) in query.iter_mut() {
        history.push(transform.translation);
    }
}

pub fn detect_rewind(mut input: EventReader<KeyboardInput>, mut game_state: ResMut<RewindState>) {
    for ev in input.iter() {
        if ev.key_code == Some(KeyCode::Space) && ev.state.is_pressed() {
            println!("triggering rewind");
            game_state.0 = true;
        }
    }
}

pub fn rewind_system(
    mut query: Query<(&mut PositionHistory, &mut Transform)>,
    mut game_state: ResMut<RewindState>,
) {
    if !game_state.0 {
        return;
    }
    let mut history_left = false;
    for (mut history, mut transform) in query.iter_mut() {
        if let Some(position) = history.pop() {
            history_left = true;
            *transform = Transform::from_translation(position);
            println!("{} iters left", history.len());
        }
    }
    if !history_left {
        println!("done with rewind");
        game_state.0 = false;
    }
}
