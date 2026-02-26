use std::collections::HashMap;

use bevy::{asset::LoadedFolder, prelude::*};
use serde::Deserialize;

use crate::{
    assets::{
        indexing::{AssetIndexPlugin, Indexable},
        loaders::toml::{FromToml, TomlAssetPlugin},
        tracking::LoadResource,
    },
    gameplay::{inventory::prelude::ItemDef, recipe::assets::Recipe},
};

pub fn plugin(app: &mut App) {
    app.add_plugins((
        TomlAssetPlugin::<StructureDef>::extensions(&["structure.toml"]),
        AssetIndexPlugin::<StructureDef>::default(),
    ));

    app.load_resource::<StructureAssets>();
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

#[derive(Deserialize)]
pub struct StructureRaw {
    pub id: String,
    pub name: String,
    pub default_recipe: Option<String>,
    #[serde(default)]
    pub cost: HashMap<String, u32>,
}

#[derive(Asset, Reflect, Debug)]
pub struct StructureDef {
    pub id: String,
    pub name: String,
    pub default_recipe: Option<AssetId<Recipe>>,
    pub cost: HashMap<Handle<ItemDef>, u32>,
}

impl FromToml for StructureDef {
    type Raw = StructureRaw;

    fn from_toml(raw: Self::Raw, load_context: &mut bevy::asset::LoadContext) -> Self {
        Self {
            id: raw.id,
            name: raw.name,
            default_recipe: raw.default_recipe.map(|recipe_id| {
                load_context
                    .load(format!("manifests/recipes/{recipe_id}.recipe.toml"))
                    .id()
            }),
            cost: raw
                .cost
                .iter()
                .map(|(key, val)| {
                    (
                        load_context.load(format!("manifests/items/{key}.item.toml")),
                        *val,
                    )
                })
                .collect(),
        }
    }
}

impl Indexable for StructureDef {
    fn index(&self) -> &String {
        &self.id
    }
}
