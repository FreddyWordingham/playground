use bevy::prelude::*;

use crate::grid::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let dirt_texture = asset_server.load("icons/tiles/outdoor/dirt.png");

    for ix in (-GRID_WIDTH / 2)..(GRID_WIDTH / 2) + 1 {
        for iy in (-GRID_HEIGHT / 2)..(GRID_HEIGHT / 2) + 1 {
            let x = ix as f32 * CELL_SIZE;
            let y = iy as f32 * CELL_SIZE;
            commands.spawn((
                Sprite::from_image(dirt_texture.clone()),
                Transform::from_translation(Vec3::new(x, y, -1.0))
                    .with_scale(Vec3::splat(CELL_SIZE * 0.001)),
            ));
        }
    }
}
