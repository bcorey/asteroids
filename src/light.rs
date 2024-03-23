use bevy::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;
use core::f32::consts::PI;
pub struct LightingPlugin;

impl Plugin for LightingPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_directional_light);
  }
}

fn spawn_directional_light(mut commands: Commands) {
  commands.spawn(DirectionalLightBundle {
    directional_light: DirectionalLight {
        shadows_enabled: true,
        ..default()
    },
    transform: Transform {
        translation: Vec3::new(0.0, 2.0, 0.0),
        rotation: Quat::from_rotation_x(-PI / 4.),
        ..default()
    },
    // The default cascade config is designed to handle large scenes.
    // As this example has a much smaller world, we can tighten the shadow
    // bounds for better visual quality.
    cascade_shadow_config: CascadeShadowConfigBuilder {
        first_cascade_far_bound: 4.0,
        maximum_distance: 10.0,
        ..default()
    }
    .into(),
    ..default()
  });
}