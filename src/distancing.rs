use bevy::prelude::*;

const MIN_DISTANCE: f32 = 20.0;

#[derive(Component)]
pub struct Distance;

pub struct DistancingPlugin;

impl Plugin for DistancingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_distancing);
    }
}

pub fn process_distancing(mut query: Query<(Entity, &mut Transform), With<Distance>>) {
    // let mut nearby_all = Vec::new();

    let mut todo_transforms = Vec::new();

    for (entity_a, transform_a) in &query {
        let mut displacement = Vec3::ZERO;

        for (entity_b, transform_b) in &query {
            if entity_a == entity_b {
                continue;
            }

            let distance = transform_a.translation.distance(transform_b.translation);
            if distance < MIN_DISTANCE {
                let direction = transform_a.translation - transform_b.translation;
                displacement += direction.normalize() * (MIN_DISTANCE - distance);
            }
        }

        todo_transforms.push((entity_a, displacement));
    }

    for (entity, displacement) in todo_transforms {
        if let Ok((_entity, mut transform)) = query.get_mut(entity) {
            transform.translation += displacement;
        }
    }
}
