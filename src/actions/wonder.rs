use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::Entropy;
use rand::Rng;

use crate::movement::Velocity;

const DISTANCE_BEFORE_PICKING_NEW_POINT: f32 = 10.0;
const WONDER_SPEED: f32 = 0.1;
const MIN_DISTANCE: f32 = 200.0;
const MAX_DISTANCE: f32 = 500.0;

#[derive(Component)]
#[require(Velocity)]
pub struct Wondering {
    target: Vec3,
}

impl Wondering {
    pub fn new(target: Vec3) -> Self {
        Self { target }
    }
}

pub struct WonderPlugin;

impl Plugin for WonderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, wondering_system);
    }
}

fn wondering_system(
    mut query: Query<(
        &mut Entropy<WyRand>,
        &mut Wondering,
        &mut Velocity,
        &Transform,
    )>,
) {
    for (mut rng, mut wondering, mut velocity, transform) in &mut query {
        // Check if entity is close to target
        let distance = transform.translation.distance(wondering.target);
        if distance < DISTANCE_BEFORE_PICKING_NEW_POINT {
            // Pick a new target
            let theta = rng.gen_range(0.0..PI * 2.0);
            let distance = rng.gen_range(MIN_DISTANCE..MAX_DISTANCE);
            let x = distance * theta.cos();
            let y = distance * theta.sin();
            wondering.target = Vec3::new(x, y, 0.0) + transform.translation;

            // Set the velocity to move towards the target
            let direction = (wondering.target - transform.translation).normalize();
            velocity.walking = direction * WONDER_SPEED;
        }
    }
}
