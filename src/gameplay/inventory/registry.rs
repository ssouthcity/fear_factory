use std::collections::HashMap;

use bevy::prelude::*;

use crate::gameplay::inventory::assets::ItemDef;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct ItemRegistry {
    pub items: HashMap<String, Handle<ItemDef>>,
}

pub fn load_item_definitions(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut registry = ItemRegistry::default();

    registry.items.insert(
        "doll".to_string(),
        asset_server.load("manifests/doll.item.toml"),
    );

    registry.items.insert(
        "fauna_a".to_string(),
        asset_server.load("manifests/fauna_a.item.toml"),
    );

    registry.items.insert(
        "flora_a".to_string(),
        asset_server.load("manifests/flora_a.item.toml"),
    );

    registry.items.insert(
        "quintessence".to_string(),
        asset_server.load("manifests/quintessence.item.toml"),
    );

    registry.items.insert(
        "ritual_doll".to_string(),
        asset_server.load("manifests/ritual_doll.item.toml"),
    );

    registry.items.insert(
        "sack".to_string(),
        asset_server.load("manifests/sack.item.toml"),
    );

    commands.insert_resource(registry);
}
