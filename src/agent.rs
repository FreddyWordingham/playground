use bevy::prelude::*;

use crate::health::Health;

#[derive(Component)]
#[require(Health)]
pub struct Agent {
    pub _name: String,
}

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, _app: &mut App) {}
}
