use bevy::prelude::*;

use crate::spaceship::Spaceship;

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, despawn_far_away_entities);
    }
}

fn despawn_far_away_entities(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Without<Spaceship>>,
) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);
        if distance > DESPAWN_DISTANCE && commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
}