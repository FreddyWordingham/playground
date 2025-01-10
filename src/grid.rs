use bevy::{color::palettes::css::*, math::Isometry2d, prelude::*};

pub const CELL_SIZE: f32 = 50.0;
pub const GRID_HEIGHT: i32 = 249;
pub const GRID_WIDTH: i32 = 249;

#[derive(Component, Default)]
#[require(Transform)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_grid_positions,
                render_grid,
                render_grid_positions,
                render_origin,
            )
                .chain(),
        );
    }
}

fn update_grid_positions(mut query: Query<(&Transform, &mut GridPosition)>) {
    for (transform, mut pos) in query.iter_mut() {
        pos.x = (transform.translation.x / CELL_SIZE).round() as i32;
        pos.y = (transform.translation.y / CELL_SIZE).round() as i32;
    }
}

fn render_grid(mut gizmos: Gizmos) {
    gizmos
        .grid_2d(
            Isometry2d::IDENTITY,
            UVec2::new(GRID_WIDTH as u32, GRID_HEIGHT as u32),
            Vec2::new(CELL_SIZE, CELL_SIZE),
            // Dark gray
            LinearRgba::gray(0.05),
        )
        .outer_edges();
}

fn render_grid_positions(mut gizmos: Gizmos, query: Query<&GridPosition>) {
    for pos in query.iter() {
        let x = pos.x as f32 * CELL_SIZE;
        let y = pos.y as f32 * CELL_SIZE;
        let isometry = Isometry2d::from_translation(Vec2::new(x, y));
        gizmos.rect_2d(isometry, Vec2::splat(CELL_SIZE), YELLOW);
    }
}

fn render_origin(mut gizmos: Gizmos) {
    gizmos.circle_2d(Isometry2d::IDENTITY, CELL_SIZE * 0.5, GREEN);
    gizmos.line_2d(
        Vec2::new(-CELL_SIZE * 0.5, 0.0),
        Vec2::new(CELL_SIZE * 0.5, 0.0),
        GREEN,
    );
    gizmos.line_2d(
        Vec2::new(0.0, -CELL_SIZE * 0.5),
        Vec2::new(0.0, CELL_SIZE * 0.5),
        GREEN,
    );
}
