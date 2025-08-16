use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::assets::{
    LoadResource,
    manifest::{Id, Manifest, ManifestPlugin},
};

use super::Item;

pub fn plugin(app: &mut App) {
    app.register_type::<ItemAssets>();

    app.add_plugins(ManifestPlugin::<Item>::default())
        .load_resource::<ItemAssets>();
}

#[derive(Asset, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct ItemAssets {
    pub manifest: Handle<Manifest<Item>>,
    pub aseprite: Handle<Aseprite>,
}

impl FromWorld for ItemAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            manifest: asset_server.load("manifest/items.toml"),
            aseprite: asset_server.load("items.aseprite"),
        }
    }
}

impl ItemAssets {
    fn ase_slice(&self, item_id: Id<Item>) -> impl Bundle {
        AseSlice {
            aseprite: self.aseprite.clone(),
            name: item_id.value,
        }
    }

    pub fn sprite(&self, item: Id<Item>) -> impl Bundle {
        (Sprite::sized(Vec2::splat(16.0)), self.ase_slice(item))
    }
}
