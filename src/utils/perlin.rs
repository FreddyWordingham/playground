use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::GlobalEntropy;
use ndarray::Array2;
use rand::Rng;

pub struct Perlin {
    vectors: Array2<Vec2>,
}

impl Perlin {
    pub fn new(size: (usize, usize), rng: &mut GlobalEntropy<WyRand>) -> Self {
        let mut vectors = Array2::from_elem((size.1, size.0), Vec2::ZERO);
        for vector in vectors.iter_mut() {
            *vector = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize();
        }
        Self { vectors }
    }

    pub fn sample(&self, position: Vec2) -> f32 {
        let x = position.x.fract();
        let y = position.y.fract();

        let gx = x * (self.vectors.shape()[1] as f32);
        let gy = y * (self.vectors.shape()[0] as f32);

        let x0 = gx.floor() as usize;
        let y0 = gy.floor() as usize;
        let x1 = (x0 + 1) % self.vectors.shape()[1];
        let y1 = (y0 + 1) % self.vectors.shape()[0];

        let v00 = self.vectors[[y0, x0]];
        let v01 = self.vectors[[y1, x0]];
        let v10 = self.vectors[[y0, x1]];
        let v11 = self.vectors[[y1, x1]];

        let p00 = Vec2::new(gx - x0 as f32, gy - y0 as f32);
        let p01 = Vec2::new(gx - x0 as f32, gy - y1 as f32);
        let p10 = Vec2::new(gx - x1 as f32, gy - y0 as f32);
        let p11 = Vec2::new(gx - x1 as f32, gy - y1 as f32);

        let d00 = v00.dot(p00);
        let d01 = v01.dot(p01);
        let d10 = v10.dot(p10);
        let d11 = v11.dot(p11);

        let fx = fade(p00.x);
        let fy = fade(p00.y);

        let d0 = d00 + fx * (d10 - d00);
        let d1 = d01 + fx * (d11 - d01);

        d0 + fy * (d1 - d0)
    }
}

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}
