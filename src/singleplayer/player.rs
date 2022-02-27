use bevy::{math::const_vec2, prelude::*};
use leafwing_input_manager::prelude::*;
use std::ops::{Add, Not};

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("images/player.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0, 64.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(MovementDirection::NotMoving)
        .insert_bundle(InputManagerBundle::<Action> {
            // Stores "which virtual action buttons are currently pressed"
            action_state: ActionState::default(),
            // Stores how those actions relate to inputs from your player
            input_map: InputMap::new([
                (Action::MoveLeft, KeyCode::A),
                (Action::MoveRight, KeyCode::D),
            ]),
        });
}

pub fn player_movement(
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut MovementDirection, &ActionState<Action>), With<Player>>,
) {
    let (mut transform, mut direction, input) = player.single_mut();
    //todo!();
    for action in input.get_just_pressed().iter() {
        *direction = *direction + MovementDirection::from_action(*action);
    }

    for action in input.get_just_released().iter() {
        *direction = *direction + !MovementDirection::from_action(*action);
    }

    transform.translation += direction.as_vec2().extend(0.) * time.delta_seconds() * 250.;
}

/// The player.
#[derive(Debug, Component, Clone, Copy)]
pub struct Player;

/// The possible keyboard button actions a player can do.
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    MoveLeft,
    MoveRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component)]
pub enum MovementDirection {
    Left,
    Right,
    NotMoving,
}

impl MovementDirection {
    pub fn from_action(action: Action) -> Self {
        use MovementDirection::*;
        match action {
            Action::MoveLeft => Left,
            Action::MoveRight => Right,
            _ => NotMoving,
        }
    }

    pub const fn as_vec2(self) -> Vec2 {
        match self {
            MovementDirection::Left => const_vec2!([-1., 0.]),
            MovementDirection::Right => const_vec2!([1., 0.]),
            MovementDirection::NotMoving => const_vec2!([0., 0.]),
        }
    }

    pub fn from_vec2(vec2: Vec2) -> Self {
        let vec2 = vec2.normalize_or_zero();
        use MovementDirection::*;
        if vec2.x >= 0.5 {
            Right
        } else if vec2.x <= -0.5 {
            Left
        } else {
            NotMoving
        }
    }
}

impl Add for MovementDirection {
    type Output = Self;

    /// Finds the overall direction headed when two directions are given.
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_vec2(self.as_vec2() + rhs.as_vec2())
    }
}

impl Not for MovementDirection {
    type Output = Self;

    fn not(self) -> Self::Output {
        use MovementDirection::*;
        match self {
            Left => Right,
            Right => Left,
            NotMoving => NotMoving,
        }
    }
}
