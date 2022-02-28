use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use bevy::prelude::*;
use heron::{CollisionShape, RigidBody, RotationConstraints, Velocity};

#[derive(Component, Debug, Default)]
pub struct BulletStats {
    _damage: f32,
}

#[derive(Bundle, Default)]
pub struct BulletBundle {
    bullet_stats: BulletStats,
    #[bundle]
    sprite: SpriteBundle,
    rb: RigidBody,
    constraints: RotationConstraints,
    collision_shape: CollisionShape,
    velocity: Velocity,
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

    pub fn create_bullet_bundle(&self, asset_server: &AssetServer) -> BulletBundle {
        let transform = Transform::from_xyz(10.0, 0.0, 1.2);
        match self {
            GunType::Shotgun => BulletBundle {
                bullet_stats: BulletStats { _damage: 3.0 },
                sprite: SpriteBundle {
                    texture: asset_server.load("images/shotgun_bullet.png"),
                    transform,
                    ..Default::default()
                },
                rb: RigidBody::Dynamic,
                constraints: RotationConstraints::lock(),
                collision_shape: CollisionShape::Cuboid {
                    half_extends: Vec3::new(8f32, 1f32, 0f32),
                    border_radius: None,
                },
                // FIXME; should calculate direction based on aim
                velocity: Velocity::from_linear(Vec3::new(50.0, 50.0, 0.0)),
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
