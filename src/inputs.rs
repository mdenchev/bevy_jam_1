use bevy::{core::FixedTimestep, prelude::*};

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInput::default())
            .add_system_to_stage(
                CoreStage::PreUpdate,
                get_player_inputs.with_run_criteria(
                    FixedTimestep::steps_per_second(60.0).with_label("update_inputs"),
                ),
            );
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlayerInput {
    pub move_direction: Vec2,
    pub aim_direction: Vec2,
    pub shoot: ButtonState,
    pub throw: ButtonState,
    pub dodge: ButtonState,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonState {
    /// Is currently not pressed, and was not just released this frame
    Up,
    /// Is currently pressed
    Down,
    /// Was pressed this frame
    Pressed,
    /// Was released this frame
    Released,
}

impl Default for ButtonState {
    fn default() -> Self {
        Self::Up
    }
}

impl ButtonState {
    #[inline]
    fn upgrade(&mut self) {
        *self = match self {
            ButtonState::Up => *self,
            ButtonState::Down => ButtonState::Released,
            ButtonState::Pressed => ButtonState::Released,
            ButtonState::Released => ButtonState::Up,
        };
    }

    #[inline]
    fn downgrade(&mut self) {
        *self = match self {
            ButtonState::Up => ButtonState::Pressed,
            ButtonState::Down => *self,
            ButtonState::Pressed => ButtonState::Down,
            ButtonState::Released => ButtonState::Pressed,
        };
    }
}

fn get_player_inputs(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut player_input: ResMut<PlayerInput>,
) {
    // Create our move vector from keyboard inputs
    let mut move_direction = Vec2::ZERO;
    move_direction.x -= if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
        1.0
    } else {
        0.0
    };
    move_direction.x += if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
        1.0
    } else {
        0.0
    };
    move_direction.y += if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
        1.0
    } else {
        0.0
    };
    move_direction.y -= if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down) {
        1.0
    } else {
        0.0
    };
    if move_direction.length_squared() != 0.0 {
        move_direction = move_direction.normalize();
    }
    player_input.move_direction = move_direction;

    // Create our aim vector
    let window = windows.get_primary().unwrap();

    if let Some(position) = window.cursor_position() {
        player_input.aim_direction =
            position - Vec2::new(window.width() / 2.0, window.height() / 2.0);
    }

    if player_input.aim_direction.length_squared() != 0.0 {
        player_input.aim_direction = player_input.aim_direction.normalize();
    }

    // Get our action states
    if keys.pressed(KeyCode::Space) {
        player_input.dodge.downgrade()
    } else {
        player_input.dodge.upgrade()
    };
    if mouse.pressed(MouseButton::Left) {
        player_input.shoot.downgrade()
    } else {
        player_input.shoot.upgrade()
    };
    if mouse.pressed(MouseButton::Right) {
        player_input.throw.downgrade()
    } else {
        player_input.shoot.upgrade()
    };
}
