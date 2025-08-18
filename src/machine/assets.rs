use bevy::{asset::LoadedFolder, prelude::*};
use serde::Deserialize;

use crate::{
    assets::{
        LoadResource,
        manifest::{Manifest, ManifestPlugin},
    },
    logistics::ConveyorHole,
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
            animations: asset_server.load("structures/"),
        }
    }
}

#[derive(Debug, TypePath, Deserialize)]
pub struct StructureTemplate {
    pub name: String,
    #[serde(default)]
    pub power: PowerTemplate,
    #[serde(default)]
    pub conveyor_holes: Vec<ConveyorHoleTemplate>,
}

#[derive(Debug, TypePath, Deserialize, Default)]
pub struct PowerTemplate {
    #[serde(default = "default_power_sockets")]
    pub sockets: u8,
    #[serde(default)]
    pub consumption: f32,
    #[serde(default)]
    pub production: f32,
}

const fn default_power_sockets() -> u8 {
    1
}

#[derive(Debug, TypePath, Deserialize)]
pub struct ConveyorHoleTemplate {
    pub direction: ConveyorHole,
    pub translation: Vec3,
}
