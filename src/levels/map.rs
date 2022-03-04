use std::time::Duration;

#[derive(Debug, Default)]
pub struct MapInitData {
    pub player_spawn_position: (f32, f32),
    pub enemy_spawn_positions: Vec<(f32, f32)>,
    // Fixme move somewhere more sensible
    pub kills: usize,
    pub timer: Duration,
}
