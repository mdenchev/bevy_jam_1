#[derive(Debug, Default)]
pub struct MapInitData {
    pub player_spawn_position: (f32, f32),
    pub enemy_spawn_positions: Vec<(f32, f32)>,
}
