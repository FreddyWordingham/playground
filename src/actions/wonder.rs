use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::Entropy;
use rand::prelude::Rng;

#[derive(Component)]
pub struct Wondering {
    speed: f32,
    direction: Vec3,
}

impl Wondering {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            direction: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

pub struct WonderPlugin;

impl Plugin for WonderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, wondering_system);
    }
}

fn wondering_system(mut query: Query<(&mut Entropy<WyRand>, &mut Wondering, &mut Transform)>) {
    for (mut rng, mut wondering, mut transform) in &mut query {
        if rng.gen_bool(0.01) {
            let new_theta = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
            wondering.direction = Vec3::new(new_theta.cos(), new_theta.sin(), 0.0);
        }
        transform.translation += wondering.direction * wondering.speed;
    }
}
