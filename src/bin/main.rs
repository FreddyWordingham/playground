use bevy::prelude::*;

use playground;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(playground::GenesisPlugin)
        .run();
}
