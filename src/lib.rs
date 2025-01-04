use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;

mod actions;
mod agent;
mod camera;
mod genesis;
mod health;
mod status;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.1, 0.3, 0.2)))
            .add_plugins(DefaultPlugins)
            .add_plugins(EntropyPlugin::<WyRand>::default())
            .add_plugins(camera::CameraPlugin)
            .add_plugins(status::StatusPlugin)
            .add_plugins(health::HealthPlugin)
            .add_plugins(actions::WonderPlugin)
            .add_plugins(actions::HuntPlugin)
            .add_plugins(agent::AgentPlugin)
            .add_plugins(genesis::GenesisPlugin);
    }
}
