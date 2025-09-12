use bevy::{asset::LoadedFolder, prelude::*};
use serde::Deserialize;

use crate::assets::{
    indexing::{AssetIndexPlugin, Indexable},
    loaders::toml::TomlAssetPlugin,
    tracking::LoadResource,
};

pub fn plugin(app: &mut App) {
    app.add_plugins((
        TomlAssetPlugin::<StructureDef>::extensions(&["structure.toml"]),
        AssetIndexPlugin::<StructureDef>::default(),
    ));

    app.register_type::<StructureAssets>()
        .load_resource::<StructureAssets>();
}

#[derive(Asset, Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct StructureAssets {
    pub sprites: Handle<LoadedFolder>,
    pub manifest_folder: Handle<LoadedFolder>,
}

impl FromWorld for StructureAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            sprites: asset_server.load_folder("sprites/structures/"),
            manifest_folder: asset_server.load_folder("manifests/structures"),
        }
    }
}

#[derive(Asset, Deserialize, Reflect)]
pub struct StructureDef {
    pub id: String,
    pub name: String,
    pub default_recipe: Option<String>,
}

impl Indexable for StructureDef {
    fn index(&self) -> &String {
        &self.id
    }
}
