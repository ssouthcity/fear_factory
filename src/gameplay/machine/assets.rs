use bevy::{asset::LoadedFolder, prelude::*};
use serde::Deserialize;

use crate::assets::{
    manifest::{Manifest, ManifestPlugin},
    tracking::LoadResource,
};

pub fn plugin(app: &mut App) {
    app.add_plugins(ManifestPlugin::<StructureTemplate>::default())
        .register_type::<StructureAssets>()
        .load_resource::<StructureAssets>();
}

#[derive(Asset, Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct StructureAssets {
    pub manifest: Handle<Manifest<StructureTemplate>>,
    pub sprites: Handle<LoadedFolder>,
}

impl FromWorld for StructureAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            manifest: asset_server.load("manifests_legacy/structures.toml"),
            sprites: asset_server.load_folder("sprites/structures/"),
        }
    }
}

#[derive(Debug, TypePath, Deserialize)]
pub struct StructureTemplate {
    pub name: String,
    #[serde(default)]
    pub recipe: Option<RecipeTemplate>,
}

#[derive(Debug, TypePath, Deserialize, Default)]
pub struct RecipeTemplate {
    pub default_recipe: Option<String>,
}
