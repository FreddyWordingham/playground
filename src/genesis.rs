use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::{ForkableRng, GlobalEntropy};

use crate::actions::{Hunting, Wondering};
use crate::distancing::Distance;
use crate::health::Health;

#[derive(Component, Default)]
#[require(Health)]
struct Plant;

#[derive(Component, Default)]
#[require(Health)]
struct Animal;

#[derive(Component, Default)]
#[require(Animal)]
pub struct Herbivore;

#[derive(Component, Default)]
#[require(Animal)]
struct Carnivore;

#[derive(Component)]
#[require(Plant)]
struct BerryBush;

#[derive(Component)]
#[require(Carnivore)]
struct Fox;

#[derive(Component)]
#[require(Herbivore)]
struct Rabbit;

pub struct GenesisPlugin;

impl Plugin for GenesisPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut global: GlobalEntropy<WyRand>,
) {
    // Bushes
    let bush_locations = vec![
        Vec3::new(200.0, 300.0, -1.0),
        Vec3::new(400.0, -100.0, -1.0),
        Vec3::new(100.0, -200.0, -1.0),
        Vec3::new(-100.0, -200.0, -1.0),
    ];
    let bush_texture = asset_server.load("icons/plants/bush/atlas.png");
    let bush_layout = TextureAtlasLayout::from_grid(UVec2::splat(128), 3, 1, None, None);
    let bush_texture_atlas_layout = texture_atlas_layouts.add(bush_layout);
    for location in bush_locations {
        commands
            .spawn((
                global.fork_rng(),
                Sprite::from_atlas_image(
                    bush_texture.clone(),
                    TextureAtlas {
                        layout: bush_texture_atlas_layout.clone(),
                        index: 0,
                    },
                ),
                Transform::from_translation(location).with_scale(Vec3::splat(0.5)),
                BerryBush,
            ))
            .with_child((
                Text2d::new("bush"),
                Transform::from_translation(Vec3::new(0.0, -85.0, 1.0)),
            ));
    }

    // Foxes
    let fox_locations = vec![
        Vec3::new(-200.0, 100.0, 0.0),
        Vec3::new(-300.0, 200.0, 0.0),
        Vec3::new(-200.0, 300.0, 0.0),
        Vec3::new(-100.0, 300.0, 0.0),
        Vec3::new(100.0, 300.0, 0.0),
        Vec3::new(200.0, 300.0, 0.0),
        Vec3::new(300.0, 200.0, 0.0),
        Vec3::new(200.0, 100.0, 0.0),
        Vec3::new(100.0, 100.0, 0.0),
    ];
    let fox_texture = asset_server.load("icons/animals/fox/atlas.png");
    let fox_layout = TextureAtlasLayout::from_grid(UVec2::splat(128), 4, 1, None, None);
    let fox_texture_atlas_layout = texture_atlas_layouts.add(fox_layout);
    for location in fox_locations {
        commands
            .spawn((
                global.fork_rng(),
                Sprite::from_atlas_image(
                    fox_texture.clone(),
                    TextureAtlas {
                        layout: fox_texture_atlas_layout.clone(),
                        index: 0,
                    },
                ),
                Transform::from_translation(location).with_scale(Vec3::splat(0.75)),
                Fox,
                Distance,
                Hunting::new(0.15),
            ))
            .with_child((
                Text2d::new("fox"),
                Transform::from_scale(Vec3::splat(0.7))
                    .with_translation(Vec3::new(0.0, -30.0, 1.0)),
            ));
    }

    // Rabbits
    let rabbit_locations = vec![
        Vec3::new(200.0, -100.0, 0.0),
        Vec3::new(300.0, -200.0, 0.0),
        Vec3::new(200.0, -300.0, 0.0),
        Vec3::new(100.0, -300.0, 0.0),
        Vec3::new(-100.0, -300.0, 0.0),
    ];
    let rabbit_texture = asset_server.load("icons/animals/rabbit/atlas.png");
    let rabbit_layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 4, 1, None, None);
    let rabbit_texture_atlas_layout = texture_atlas_layouts.add(rabbit_layout);
    for location in rabbit_locations {
        commands
            .spawn((
                global.fork_rng(),
                Sprite::from_atlas_image(
                    rabbit_texture.clone(),
                    TextureAtlas {
                        layout: rabbit_texture_atlas_layout.clone(),
                        index: 0,
                    },
                ),
                Transform::from_translation(location).with_scale(Vec3::splat(1.0)),
                Rabbit,
                Wondering::new(0.2),
            ))
            .with_child((
                Text2d::new("rabbit"),
                Transform::from_scale(Vec3::splat(0.5))
                    .with_translation(Vec3::new(0.0, -25.0, 1.0)),
            ));
    }
}
