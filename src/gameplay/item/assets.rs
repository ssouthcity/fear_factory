use bevy::{asset::LoadedFolder, prelude::*};
use serde::Deserialize;

use crate::assets::{
    indexing::{AssetIndexPlugin, Indexable},
    loaders::toml::TomlAssetPlugin,
    tracking::LoadResource,
};

pub fn plugin(app: &mut App) {
    app.add_plugins((
        TomlAssetPlugin::<ItemDef>::extensions(&["item.toml"]),
        AssetIndexPlugin::<ItemDef>::default(),
    ));

    app.load_resource::<ItemAssets>();
}

#[derive(Asset, Clone, Debug, Deserialize, Reflect)]
pub struct ItemDef {
    pub id: String,
    pub name: String,
    #[serde(default = "placeholder_sprite")]
    pub sprite: String,
    pub stack_size: u32,
    pub taxonomy: Taxonomy,
    pub transport: Transport,
}

#[derive(Component, Clone, Debug, Deserialize, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub enum Taxonomy {
    Fauna,
    Flora,
    Minerale,
}

#[derive(Clone, Debug, Deserialize, Reflect)]
pub enum Transport {
    Box,
    Bag,
}

fn placeholder_sprite() -> String {
    String::from("sprites/items/placeholder.png")
}

impl Indexable for ItemDef {
    fn index(&self) -> &String {
        &self.id
    }
}

#[derive(Asset, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct ItemAssets {
    pub item_definitions: Handle<LoadedFolder>,
    pub item_sprites: Handle<LoadedFolder>,
}

impl FromWorld for ItemAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            item_definitions: asset_server.load_folder("manifests/items"),
            item_sprites: asset_server.load_folder("sprites/items"),
        }
    }
}
