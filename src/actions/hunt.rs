use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::Entropy;
use rand::prelude::Rng;

use crate::{genesis::Herbivore, health::DamageEvent, movement::Velocity};

const CHASE_SPEED: f32 = 0.2;
const ATTACK_DISTANCE: f32 = 10.0;
const ATTACK_CHANCE: f32 = 0.01;

#[derive(Component)]
#[require(Velocity)]
pub struct Hunting {
    target: Option<Entity>,
}

impl Hunting {
    pub fn new() -> Self {
        Self { target: None }
    }
}

pub struct HuntPlugin;

impl Plugin for HuntPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hunting_system);
    }
}

fn hunting_system(
    mut query: Query<(
        &mut Entropy<WyRand>,
        &mut Hunting,
        &mut Velocity,
        &Transform,
    )>,
    targets: Query<(Entity, &Transform), (With<Herbivore>, Without<Hunting>)>,
    mut damage_queue: ResMut<Events<DamageEvent>>,
) {
    for (mut rng, mut hunting, mut velocity, transform) in &mut query {
        // If there is a target, move towards it and attack it when close enough
        if let Some(target) = hunting.target {
            if let Ok((_entity, target_transform)) = targets.get(target) {
                let delta = target_transform.translation - transform.translation;
                let distance = delta.length();
                if distance > ATTACK_DISTANCE {
                    let direction = delta / distance;
                    velocity.walking = direction * CHASE_SPEED;
                } else {
                    if rng.gen_bool(ATTACK_CHANCE as f64) {
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
