use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::GlobalEntropy;

use crate::utils::Perlin;

#[derive(Clone)]
pub struct LayerConfig {
    resolution: (usize, usize),
    amplitude: f32,
    frequency: f32,
}

impl LayerConfig {
    pub fn new(resolution: (usize, usize), amplitude: f32, frequency: f32) -> Self {
        Self {
            resolution,
            amplitude,
            frequency,
        }
    }
}

struct PerlinLayer {
    perlin: Perlin,
    amplitude: f32,
    frequency: f32,
}

pub struct MultiLayerPerlin {
    layers: Vec<PerlinLayer>,
}

impl MultiLayerPerlin {
    pub fn new(configs: &[LayerConfig], rng: &mut GlobalEntropy<WyRand>) -> Self {
        let mut layers = Vec::new();
        for config in configs {
            layers.push(PerlinLayer {
                perlin: Perlin::new(config.resolution, rng),
                amplitude: config.amplitude,
                frequency: config.frequency,
            });
        }
        Self { layers }
    }

    pub fn sample(&self, position: Vec2) -> f32 {
        self.layers
            .iter()
            .map(|layer| {
                let scaled_pos = position * layer.frequency;
                layer.perlin.sample(scaled_pos) * layer.amplitude
            })
            .sum()
    }
}
