use bevy::prelude::*;
use rand::Rng;
use crate::collision_detection::Collider;
use crate::movement::Acceleration;
use crate::movement::Velocity;
use crate::movement::MovingObjectBundle;
use crate::asset_loader::SceneAssets;
use std::ops::Range;

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;
const ROTATE_SPEED: f32 = 2.0;
const RADIUS: f32 = 2.5;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
  timer: Timer,
}

pub struct AsteroidPlugin;
impl Plugin for AsteroidPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(SpawnTimer {
      timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
    })
    .add_systems(Update, (
      spawn_asteroid,
      rotate_asteroids,
      handle_asteroid_collisions,
    ));
  }
}

fn spawn_asteroid(
  mut commands: Commands,
  mut spawn_timer: ResMut<SpawnTimer>,
  time: Res<Time>,
  scene_assets: Res<SceneAssets>
) {
  spawn_timer.timer.tick(time.delta());
  if !spawn_timer.timer.just_finished() {
    return;
  }

  let mut rng = rand::thread_rng();
  let translation = Vec3::new(
    rng.gen_range(SPAWN_RANGE_X),
    0.0,
    rng.gen_range(SPAWN_RANGE_Z),
  );

  let mut random_unit_vector = || Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0)).normalize_or_zero();
  let velocity = random_unit_vector() * VELOCITY_SCALAR;
  let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

  let asteroid_scale = rng.gen_range(1.0..3.0);
  let asteroid_transform = Transform {
    translation,
    rotation: Quat::from_rotation_y(rng.gen_range(0.0..4.0)),
    scale: Vec3::splat(asteroid_scale),
  };

  commands.spawn((
    MovingObjectBundle {
      velocity: Velocity::new(velocity),
      acceleration: Acceleration::new(acceleration),
      collider: Collider::new(RADIUS * asteroid_scale),
      model: SceneBundle {
        scene: scene_assets.asteroid.clone(),
        transform: asteroid_transform,
        ..default()
      },
    },
    Asteroid,
  ));
}

fn rotate_asteroids(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
  for mut transform in query.iter_mut() {
    transform.rotate_local_z(ROTATE_SPEED * time.delta_seconds());
  }
}

fn handle_asteroid_collisions(
  mut commands: Commands,
  query: Query<(Entity, &Collider), With<Asteroid>>,
) {
  for (entity, collider) in query.iter() {
    for &collided_entity in collider.colliding_entities.iter() {
      // Asteroid hit another asteroid. query contains only asteroids.
      if query.get(collided_entity).is_ok() {
        continue;
      }
      // Asteroid hits something else
      commands.entity(entity).despawn_recursive();
    }
  }
}