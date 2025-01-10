use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::GlobalEntropy;
use ndarray::Array2;
use rand::Rng;

use crate::grid::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH};
use crate::utils::{LayerConfig, MultiLayerPerlin};

#[derive(Clone)]
pub enum Tile {
    Stone,
    Dirt,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut global: GlobalEntropy<WyRand>,
) {
    // Create noise for the map
    let layer_configs = vec![
        LayerConfig::new((4, 4), 1.0, 1.0),
        LayerConfig::new((8, 8), 0.5, 2.0),
        LayerConfig::new((16, 16), 0.25, 4.0),
    ];
    let noise = MultiLayerPerlin::new(&layer_configs, &mut global);

    // Load textures for each tile type
    let dirt_texture = asset_server.load("icons/tiles/outdoor/dirt.png");
    let stone_texture = asset_server.load("icons/tiles/outdoor/stone/smooth.png");

    // Create a 2D grid of tiles
    let mut tiles = Array2::from_elem((GRID_HEIGHT as usize, GRID_WIDTH as usize), Tile::Stone);

    // Determine type of each tile
    for ((ix, iy), tile) in tiles.indexed_iter_mut() {
        let x = ix as f32 / GRID_WIDTH as f32;
        let y = iy as f32 / GRID_HEIGHT as f32;
        let value = noise.sample(Vec2::new(x, y));

        // Choose a tile type
        *tile = if value > 0.0 { Tile::Stone } else { Tile::Dirt };
    }

    for ((ix, iy), tile) in tiles.indexed_iter() {
        let x = (ix as i32 - (GRID_WIDTH / 2)) as f32 * CELL_SIZE;
        let y = (iy as i32 - (GRID_HEIGHT / 2)) as f32 * CELL_SIZE;

        // Choose the correct texture based on the tile variant
        let texture = match tile {
            Tile::Stone => stone_texture.clone(),
            Tile::Dirt => dirt_texture.clone(),
        };

        commands.spawn((
            Sprite::from_image(texture),
            Transform::from_translation(Vec3::new(x, y, -1.0))
                .with_scale(Vec3::splat(CELL_SIZE * 0.001)),
        ));
    }
}
