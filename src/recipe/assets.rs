use bevy::prelude::*;

use crate::{
    assets::{
        LoadResource,
        manifest::{Manifest, ManifestPlugin},
    },
    recipe::Recipe,
};

pub fn plugin(app: &mut App) {
    app.add_plugins(ManifestPlugin::<Recipe>::default())
        .register_type::<RecipeAssets>()
        .load_resource::<RecipeAssets>();
}

#[derive(Asset, Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct RecipeAssets {
    pub manifest: Handle<Manifest<Recipe>>,
}

impl FromWorld for RecipeAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            manifest: assets.load("manifest/recipes.toml"),
        }
    }
}
