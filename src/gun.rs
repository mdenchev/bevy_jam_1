use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use bevy::prelude::*;
use heron::{
    rapier_plugin::{convert::IntoRapier, rapier2d::prelude::RigidBodySet, RigidBodyHandle},
    CollisionEvent, CollisionShape, RigidBody, RotationConstraints, Velocity,
};

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enable_bullet_ccd)
            .add_system(despawn_on_collision);
    }
}

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
            GunType::Shotgun => Duration::from_millis(700),
        }
    }

    pub fn velocity(&self) -> f32 {
        match self {
            GunType::Shotgun => 1500.0,
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

    pub fn create_bullet_bundle(
        &self,
        asset_server: &AssetServer,
        origin: Vec3,
        aim_direction: Vec2,
    ) -> BulletBundle {
        let aim_direction = aim_direction.extend(0.0);
        let transform = Transform {
            translation: origin + aim_direction * 12.0 + Vec3::Z * 1.2,
            rotation: Quat::from_axis_angle(Vec3::Z, aim_direction.y.atan2(aim_direction.x)),
            ..Default::default()
        };
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
                velocity: Velocity::from_linear(aim_direction * self.velocity()),
            },
        }
    }

    pub fn play_sfx(
        &self,
        audio: &bevy_kira_audio::Audio,
        channel: &bevy_kira_audio::AudioChannel,
        asset_server: &AssetServer,
    ) {
        match self {
            GunType::Shotgun => {
                audio.play_in_channel(asset_server.load("sfx/shotgun.wav"), channel)
            }
        };
    }
}

fn enable_bullet_ccd(
    mut rigid_bodies: ResMut<RigidBodySet>,
    new_handles: Query<&RigidBodyHandle, (With<BulletStats>, Added<RigidBodyHandle>)>,
) {
    for handle in new_handles.iter() {
        if let Some(body) = rigid_bodies.get_mut(handle.into_rapier()) {
            body.enable_ccd(true);
        }
    }
}

fn despawn_on_collision(mut commands: Commands, mut events: EventReader<CollisionEvent>) {
    events.iter().filter(|e| e.is_started()).for_each(|ev| {
        let (e1, e2) = ev.rigid_body_entities();
        let (l1, l2) = ev.collision_layers();
        use crate::GameLayers::*;
        if l1.contains_group(World) && l2.contains_group(Bullets) {
            commands.entity(e2).despawn();
        } else if l1.contains_group(Bullets) && l2.contains_group(World) {
            commands.entity(e1).despawn();
        }
    });
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
        Self(Timer::new(Duration::from_millis(1), false)) // 0 causes a panic
    }
}
