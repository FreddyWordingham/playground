use bevy::prelude::*;

const CELL_SIZE: f32 = 50.0;
const GRID_HEIGHT: u32 = 32;
const GRID_WIDTH: u32 = 48;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_grid);
    }
}

pub fn render_grid(mut gizmos: Gizmos) {
    gizmos
        .grid_2d(
            Isometry2d::IDENTITY,
            UVec2::new(GRID_WIDTH, GRID_HEIGHT),
            Vec2::new(CELL_SIZE, CELL_SIZE),
            // Dark gray
            LinearRgba::gray(0.05),
        )
        .outer_edges();
}
