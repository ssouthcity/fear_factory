use bevy::{asset::LoadedFolder, prelude::*};
use serde::Deserialize;

use crate::assets::{
    indexing::{AssetIndexPlugin, Indexable},
    loaders::toml::{FromToml, TomlAssetPlugin},
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
pub struct ItemRaw {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default = "placeholder_sprite")]
    pub sprite: String,
    pub stack_size: u32,
    pub taxonomy: Taxonomy,
    pub transport: Transport,
}

fn placeholder_sprite() -> String {
    String::from("sprites/items/placeholder.png")
}

#[derive(Asset, Reflect, Debug)]
pub struct ItemDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub sprite: AssetId<Image>,
    pub stack_size: u32,
    pub taxonomy: Taxonomy,
    pub transport: Transport,
}

impl FromToml for ItemDef {
    type Raw = ItemRaw;

    fn from_toml(raw: Self::Raw, load_context: &mut bevy::asset::LoadContext) -> Self {
        Self {
            id: raw.id,
            name: raw.name,
            description: raw.description,
            sprite: load_context.load(raw.sprite).id(),
            stack_size: raw.stack_size,
            taxonomy: raw.taxonomy,
            transport: raw.transport,
        }
    }
}

impl Indexable for ItemDef {
    fn index(&self) -> &String {
        &self.id
    }
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
