use bevy::prelude::*;
use serde::Deserialize;

use crate::assets::manifest::Manifest;

mod manifest;

pub fn plugin(app: &mut App) {
    app.add_plugins(manifest::ManifestPlugin::<Item>::default());

    app.add_systems(Startup, load_manifest)
        .add_systems(Update, debug_manifest);
}

#[derive(Debug, Deserialize, TypePath)]
pub struct Item {
    name: String,
    stack_size: u32,
}

#[derive(Resource)]
struct ItemManifestHandle(Handle<Manifest<Item>>);

fn load_manifest(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("manifest/items.toml");
    commands.insert_resource(ItemManifestHandle(handle));
}

fn debug_manifest(handle: Res<ItemManifestHandle>, assets: Res<Assets<Manifest<Item>>>) {
    if let Some(manifest) = assets.get(&handle.0) {
        for (id, item) in &manifest.items {
            info!("Item ID: {}, {:?}", id, item);
        }
    }
}
