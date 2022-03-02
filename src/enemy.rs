use bevy::{core::FixedTimestep, prelude::*};
use heron::prelude::*;

use crate::{player::PlayerStats, utils::CommonHandles};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(crate::GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(60.0))
                .with_system(enemy_follow_player)
                .with_system(despawn_enemy_on_collision),
        );
    }
}

#[derive(Component)]
pub struct EnemyStats {
    pub damage: f32,
    pub speed: f32,
}

pub fn enemy_follow_player(
    players: Query<(&Transform, &PlayerStats)>,
    mut enemies: Query<(&mut Velocity, &Transform, &EnemyStats)>,
) {
    for (mut vel, enemy_trans, enemy_stats) in enemies.iter_mut() {
        // println!("ticking enemy at {:?}", enemy_trans.translation);
        // Find closest player pos
        if let Some((closest_player_trans, _)) = players.iter().min_by_key(|(player_trans, _)| {
            let dist = enemy_trans
                .translation
                .distance_squared(player_trans.translation)
                * 1000.0;
            dist as i32
        }) {
            let direction =
                (enemy_trans.translation - closest_player_trans.translation).normalize();
            // Make the enemy go there
            vel.linear = direction * enemy_stats.speed * -1.0;
        }
    }
}

pub fn spawn_enemy(commands: &mut Commands, common_handles: &Res<CommonHandles>, position: Vec2) {
    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(40),
            texture_atlas: common_handles.player_sprites.clone(),
            transform: Transform::from_translation(position.extend(1.0)),
            ..Default::default()
        })
        .insert(EnemyStats {
            damage: 50.0,
            speed: 30.0,
        })
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        .insert(CollisionShape::Sphere { radius: 10.0 })
        .insert(
            CollisionLayers::none()
                .with_group(crate::GameLayers::Enemies)
                .with_masks(&[
                    crate::GameLayers::World,
                    crate::GameLayers::Player,
                    crate::GameLayers::Bullets,
                ]),
        )
        .insert(Velocity::default());
}

fn despawn_enemy_on_collision(mut commands: Commands, mut events: EventReader<CollisionEvent>) {
    events.iter().filter(|e| e.is_started()).for_each(|ev| {
        let (e1, e2) = ev.rigid_body_entities();
        let (l1, l2) = ev.collision_layers();
        use crate::GameLayers::*;
        if l1.contains_group(Enemies) && l2.contains_group(Bullets) {
            commands.entity(e1).despawn();
            commands.entity(e2).despawn();
        } else if l1.contains_group(Bullets) && l2.contains_group(Enemies) {
            commands.entity(e1).despawn();
            commands.entity(e2).despawn();
        }
    });
}
