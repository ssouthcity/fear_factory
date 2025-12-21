use bevy::prelude::*;
use rand::seq::SliceRandom;
use serde::Deserialize;

use crate::{
    assets::{loaders::toml::TomlAssetPlugin, tracking::LoadResource},
    gameplay::random::Seed,
    screens::Screen,
};

const REFILL_THRESHOLD: usize = 50;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TomlAssetPlugin::<Names>::extensions(&["toml"]));

    app.init_asset::<Names>();

    app.load_resource::<NameAssets>();

    app.init_resource::<NameManager>();

    app.add_systems(
        Update,
        load_names.run_if(in_state(Screen::Gameplay).and(name_queue_low)),
    );
}

#[derive(Asset, Reflect, Deserialize)]
pub struct Names {
    pub neutral_names: Vec<String>,
}

#[derive(Asset, Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct NameAssets {
    names: Handle<Names>,
}

impl FromWorld for NameAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            names: asset_server.load("names.toml"),
        }
    }
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct NameManager {
    name_queue: Vec<String>,
}

impl NameManager {
    const DEFAULT_NAME: &'static str = "Unknown";

    pub fn next(&mut self) -> String {
        self.name_queue
            .pop()
            .unwrap_or(String::from(Self::DEFAULT_NAME))
    }
}

fn name_queue_low(name_manager: Res<NameManager>) -> bool {
    name_manager.name_queue.len() <= REFILL_THRESHOLD
}

fn load_names(
    mut name_manager: ResMut<NameManager>,
    name_assets: Res<NameAssets>,
    names: Res<Assets<Names>>,
    mut seed: ResMut<Seed>,
) {
    if let Some(name_collections) = names.get(&name_assets.names) {
        let mut shuffled = name_collections.neutral_names.clone();
        shuffled.shuffle(&mut seed);
        name_manager.name_queue.append(&mut shuffled);
    }
}
