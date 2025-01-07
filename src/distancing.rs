use bevy::prelude::*;

use crate::movement::Velocity;

const PREFERRED_DISTANCE: f32 = 50.0;
const PRESSURE: f32 = 10.0;

#[derive(Component)]
pub struct Distance;

pub struct DistancingPlugin;

impl Plugin for DistancingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_distancing);
    }
}

pub fn process_distancing(
    transform_query: Query<(Entity, &Transform), With<Distance>>,
    mut velocity_query: Query<&mut Velocity>,
) {
    for (entity_a, transform_a) in &transform_query {
        for (entity_b, transform_b) in &transform_query {
            if entity_a == entity_b {
                continue;
            }

            let distance = transform_a.translation.distance(transform_b.translation);
            if distance < PREFERRED_DISTANCE {
                let repulsion_strength = PRESSURE * (1.0 - distance / PREFERRED_DISTANCE);
                let direction = (transform_a.translation - transform_b.translation).normalize();
                if let Ok(mut velocity) = velocity_query.get_mut(entity_a) {
                    velocity.distancing = direction * repulsion_strength;
                }
            }
        }
    }
}
