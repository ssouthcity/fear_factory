use std::{collections::HashMap, time::Duration};

use bevy::{
    asset::{LoadContext, LoadedFolder},
    prelude::*,
};
use serde::Deserialize;

use crate::{
    assets::{
        indexing::{AssetIndexPlugin, Indexable},
        loaders::toml::{FromToml, TomlAssetPlugin},
        tracking::LoadResource,
    },
    gameplay::inventory::prelude::*,
};

pub fn plugin(app: &mut App) {
    app.add_plugins((
        TomlAssetPlugin::<Recipe>::extensions(&["recipe.toml"]),
        AssetIndexPlugin::<Recipe>::default(),
    ));

    app.load_resource::<RecipeAssets>();
}

#[derive(Deserialize)]
pub struct RecipeRaw {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub input: HashMap<String, u32>,
    #[serde(default)]
    pub output: HashMap<String, u32>,
    #[serde(with = "humantime_serde")]
    pub duration: Duration,
}

#[derive(Asset, Reflect, Debug)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub input: HashMap<AssetId<ItemDef>, u32>,
    pub output: HashMap<AssetId<ItemDef>, u32>,
    pub duration: Duration,
}

impl FromToml for Recipe {
    type Raw = RecipeRaw;

    fn from_toml(raw: Self::Raw, load_context: &mut LoadContext) -> Self {
        Self {
            id: raw.id,
            name: raw.name,
            duration: raw.duration,
            input: raw
                .input
                .iter()
                .map(|(key, &value)| {
                    let handle: Handle<ItemDef> =
                        load_context.load(format!("manifests/items/{key}.item.toml"));
                    (handle.id(), value)
                })
                .collect(),
            output: raw
                .output
                .iter()
                .map(|(key, &value)| {
                    let handle: Handle<ItemDef> =
                        load_context.load(format!("manifests/items/{key}.item.toml"));
                    (handle.id(), value)
                })
                .collect(),
        }
    }
}

impl Indexable for Recipe {
    fn index(&self) -> &String {
        &self.id
    }
}

#[derive(Asset, Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct RecipeAssets {
    pub recipe_folder: Handle<LoadedFolder>,
}

impl FromWorld for RecipeAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            recipe_folder: assets.load_folder("manifests/recipes/"),
        }
    }
}
