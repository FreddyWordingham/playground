use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::Entropy;
use rand::prelude::Rng;

use crate::{genesis::Herbivore, health::DamageEvent};

#[derive(Component)]
pub struct Hunting {
    target: Option<Entity>,
    speed: f32,
}

impl Hunting {
    pub fn new(speed: f32) -> Self {
        Self {
            target: None,
            speed,
        }
    }
}

pub struct HuntPlugin;

impl Plugin for HuntPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hunting_system);
    }
}

fn hunting_system(
    mut query: Query<(&mut Entropy<WyRand>, &mut Hunting, &mut Transform)>,
    targets: Query<(Entity, &Transform), (With<Herbivore>, Without<Hunting>)>,
    mut damage_queue: ResMut<Events<DamageEvent>>,
) {
    for (mut rng, mut hunting, mut transform) in &mut query {
        // If there is a target, move towards it and attack it when close enough
        if let Some(target) = hunting.target {
            if let Ok((_entity, target_transform)) = targets.get(target) {
                let direction = target_transform.translation - transform.translation;
                let distance = direction.length();
                if distance > 10.0 {
                    let direction = direction / distance;
                    transform.translation += direction * hunting.speed;
                } else {
                    if rng.gen_bool(0.01) {
                        let event = DamageEvent::new(target, 100);
                        damage_queue.send(event);
                    }
                }
            } else {
                hunting.target = None;
            }
        } else {
            // If there is no target, find a new target
            // Find nearest herbivore
            let mut nearest = None;
            let mut nearest_distance = f32::INFINITY;
            for (entity, target_transform) in &targets {
                let direction = target_transform.translation - transform.translation;
                let distance = direction.length();
                if distance < nearest_distance {
                    nearest = Some(entity);
                    nearest_distance = distance;
                }
            }
            hunting.target = nearest;
        }
    }
}
