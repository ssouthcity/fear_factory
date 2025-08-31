use bevy::{asset::LoadedFolder, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use serde::Deserialize;

use crate::assets::{
    LoadResource,
    indexing::{AssetIndexPlugin, Indexable},
    loaders::toml::TomlAssetPlugin,
};

pub fn plugin(app: &mut App) {
    app.register_type::<ItemDef>();
    app.add_plugins((
        TomlAssetPlugin::<ItemDef>::extensions(&["item.toml"]),
        AssetIndexPlugin::<ItemDef>::default(),
    ));

    app.register_type::<ItemAssets>();
    app.load_resource::<ItemAssets>();
}

#[derive(Asset, Clone, Debug, Deserialize, Reflect)]
pub struct ItemDef {
    pub id: String,
    pub name: String,
    pub stack_size: u32,
}

impl Indexable for ItemDef {
    fn index(&self) -> &String {
        &self.id
    }
}

#[derive(Asset, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct ItemAssets {
    pub aseprite: Handle<Aseprite>,
    pub item_definitions: Handle<LoadedFolder>,
}

impl FromWorld for ItemAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            aseprite: asset_server.load("items.aseprite"),
            item_definitions: asset_server.load_folder("items"),
        }
    }
}

impl ItemAssets {
    fn ase_slice(&self, item_id: String) -> impl Bundle {
        AseSlice {
            aseprite: self.aseprite.clone(),
            name: item_id,
        }
    }

    pub fn sprite(&self, item: String) -> impl Bundle {
        (Sprite::sized(Vec2::splat(16.0)), self.ase_slice(item))
    }
}
