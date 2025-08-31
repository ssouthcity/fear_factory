use bevy::{asset::LoadedFolder, prelude::*};
use serde::Deserialize;

use crate::{
    assets::{
        LoadResource,
        manifest::{Manifest, ManifestPlugin},
    },
    simulation::logistics::ConveyorHole,
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
    pub animations: Handle<LoadedFolder>,
}

impl FromWorld for StructureAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            manifest: asset_server.load("manifest/structures.toml"),
            animations: asset_server.load_folder("structures"),
        }
    }
}

#[derive(Debug, TypePath, Deserialize)]
pub struct StructureTemplate {
    pub name: String,
    #[serde(default)]
    pub power: Option<PowerTemplate>,
    #[serde(default)]
    pub recipe: Option<RecipeTemplate>,
    #[serde(default)]
    pub conveyor_holes: Vec<ConveyorHoleTemplate>,
}

#[derive(Debug, TypePath, Deserialize, Default)]
pub struct PowerTemplate {
    #[serde(default)]
    pub sockets: Option<u8>,
    #[serde(default)]
    pub consumption: Option<f32>,
    #[serde(default)]
    pub production: Option<f32>,
}

#[derive(Debug, TypePath, Deserialize, Default)]
pub struct RecipeTemplate {
    pub default_recipe: Option<String>,
}

#[derive(Debug, TypePath, Deserialize)]
pub struct ConveyorHoleTemplate {
    pub direction: ConveyorHole,
    pub translation: Vec3,
}
