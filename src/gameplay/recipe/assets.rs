use std::{collections::HashMap, time::Duration};

use bevy::{asset::LoadedFolder, prelude::*};
use serde::Deserialize;

use crate::assets::{
    indexing::{AssetIndexPlugin, Indexable},
    loaders::toml::TomlAssetPlugin,
    tracking::LoadResource,
};

pub fn plugin(app: &mut App) {
    app.add_plugins((
        TomlAssetPlugin::<RecipeDef>::extensions(&["recipe.toml"]),
        AssetIndexPlugin::<RecipeDef>::default(),
    ));

    app.load_resource::<RecipeAssets>();
}

#[derive(Asset, Reflect, Deserialize)]
pub struct RecipeDef {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub input: HashMap<String, u32>,
    #[serde(default)]
    pub output: HashMap<String, u32>,
    #[serde(with = "humantime_serde")]
    pub duration: Duration,
    #[serde(default)]
    pub tags: Vec<RecipeTags>,
}

impl Indexable for RecipeDef {
    fn index(&self) -> &String {
        &self.id
    }
}

#[derive(Debug, Reflect, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum RecipeTags {
    StructureId(String),
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
