use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity {
    pub walking: Vec3,
    pub distancing: Vec3,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_movement);
    }
}

pub fn process_movement(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in &mut query {
        transform.translation += velocity.walking + velocity.distancing;
    }
}
