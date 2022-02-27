use bevy::{prelude::*, core::FixedTimestep};

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin{
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerInput::default())
            .add_system_to_stage(
                CoreStage::PreUpdate,
                get_player_inputs
                    .with_run_criteria(FixedTimestep::steps_per_second(60.0)
                    .with_label("update_inputs")
                ));
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlayerInput{
    move_direction: Vec2,
    aim_direction: Vec2,
    shoot: ButtonState,
    throw: ButtonState,
    dodge: ButtonState,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonState{
    /// Is currently not pressed, and was not just released this frame
    Up,
    /// Is currently pressed
    Down,
    /// Was pressed this frame
    Pressed,
    /// Was released this frame
    Released,
}

impl Default for ButtonState{
    fn default() -> Self {
        Self::Up
    }
}

impl ButtonState{
    #[inline]
    fn upgrade(&mut self){
        *self = match self{
            ButtonState::Up => *self,
            ButtonState::Down => ButtonState::Released,
            ButtonState::Pressed => ButtonState::Released,
            ButtonState::Released => ButtonState::Up,
        };
    }

    #[inline]
    fn downgrade(&mut self){
        *self = match self{
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
){
    // Create our move vector from keyboard inputs
    let mut move_direction = Vec2::ZERO;
    move_direction.x += if keys.pressed(KeyCode::A) {1.0} else {0.0};
    move_direction.x -= if keys.pressed(KeyCode::D) {1.0} else {0.0};
    move_direction.y += if keys.pressed(KeyCode::W) {1.0} else {0.0};
    move_direction.y -= if keys.pressed(KeyCode::S) {1.0} else {0.0};
    if player_input.move_direction.length_squared() != 0.0 {
        player_input.move_direction = move_direction.normalize();
    }

    // Create our aim vector
    let window = windows.get_primary().unwrap();

    if let Some(position) = window.cursor_position() {
        player_input.aim_direction = position - Vec2::new(window.width()/2.0, window.height()/2.0);
    }
    
    if player_input.aim_direction.length_squared() != 0.0 {
        player_input.aim_direction = player_input.aim_direction.normalize();
    }


    // Get our action states
    if keys.pressed(KeyCode::Space) {player_input.dodge.downgrade()} else {player_input.dodge.upgrade()};
    if mouse.pressed(MouseButton::Left) {player_input.shoot.downgrade()} else {player_input.shoot.upgrade()};
    if mouse.pressed(MouseButton::Right) {player_input.throw.downgrade()} else {player_input.shoot.upgrade()};

    info!("{:?}", *player_input);
}