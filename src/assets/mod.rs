use bevy::prelude::*;
use serde::Deserialize;

use crate::assets::manifest::ManifestParam;

mod manifest;

pub fn plugin(app: &mut App) {
    app.add_plugins(manifest::ManifestPlugin::<Item>::new("manifest/items.toml"));

    app.add_systems(Update, debug_manifest);
}

#[derive(Debug, Deserialize, TypePath)]
pub struct Item {
    name: String,
    stack_size: u32,
}

fn debug_manifest(manifest: ManifestParam<Item>) {
    let Some(manifest) = manifest.get() else {
        return;
    };

    for (id, item) in &manifest.items {
        info!("Item ID: {}, {:?}", id, item);
    }
}
