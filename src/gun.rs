use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Bullet {
    damage: f32,
}

#[derive(Bundle, Default)]
pub struct BulletBundle {
    bullet: Bullet,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Debug, Component, Clone, Copy)]
pub enum GunType {
    Shotgun,
}

impl Default for GunType {
    fn default() -> Self {
        Self::Shotgun
    }
}

impl GunType {
    pub fn cooldown(&self) -> Duration {
        match self {
            GunType::Shotgun => Duration::from_millis(400),
        }
    }

    pub fn create_bundle(&self, asset_server: &AssetServer) -> GunBundle {
        let transform = Transform::from_xyz(10.0, 0.0, 1.1);
        match self {
            GunType::Shotgun => GunBundle {
                gun_type: *self,
                sprite: SpriteBundle {
                    texture: asset_server.load("images/shotgun.png"),
                    transform,
                    ..Default::default()
                },
                gun_timer: GunTimer::default(),
            },
        }
    }
}

#[derive(Bundle, Default)]
pub struct GunBundle {
    gun_type: GunType,
    #[bundle]
    sprite: SpriteBundle,
    gun_timer: GunTimer,
}

#[derive(Component)]
pub struct GunTimer(Timer);

impl Deref for GunTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GunTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for GunTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(1), true)) // 0 causes a panic
    }
}
