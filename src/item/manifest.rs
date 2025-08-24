use std::collections::HashMap;

use bevy::prelude::*;
use serde::Deserialize;

use crate::assets::{
    LoadResource,
    manifest::{Id, Manifest},
};

pub fn plugin(app: &mut App) {
    app.register_type::<ItemEntities>();
    app.register_type::<Item>();
    app.register_type::<StackSize>();

    app.register_type::<ItemManifestHandle>();
    app.init_resource::<ItemManifestHandle>();
    app.load_resource::<ItemManifestHandle>();

    app.register_type::<ItemEntityTracker>();
    app.init_resource::<ItemEntityTracker>();

    app.add_systems(Startup, spawn_parent_container);
    app.add_systems(
        Update,
        spawn_item.run_if(on_event::<AssetEvent<Manifest<Item>>>),
    );
}

#[derive(Bundle, Asset, Clone, Debug, Deserialize, Reflect)]
pub struct Item {
    pub name: Name,
    pub stack_size: StackSize,
}

#[derive(Component, Clone, Reflect, Debug, Deserialize)]
#[reflect(Component)]
pub struct StackSize(pub u32);

#[derive(Resource, Asset, Reflect, Debug, Clone)]
struct ItemManifestHandle(Handle<Manifest<Item>>);

impl FromWorld for ItemManifestHandle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("manifest/items.toml"))
    }
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct ItemEntityTracker(HashMap<Id<Item>, Entity>);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
struct ItemEntities;

fn spawn_parent_container(mut commands: Commands) {
    commands.spawn((Name::new("Items"), ItemEntities));
}

fn spawn_item(
    mut events: EventReader<AssetEvent<Manifest<Item>>>,
    assets: Res<Assets<Manifest<Item>>>,
    mut item_definition_tracker: ResMut<ItemEntityTracker>,
    container: Single<Entity, With<ItemEntities>>,
    mut commands: Commands,
) {
    for event in events.read() {
        let asset_id = match event {
            AssetEvent::Added { id } | AssetEvent::Modified { id } => id,
            _ => continue,
        };

        let Some(asset) = assets.get(*asset_id) else {
            continue;
        };

        for (item_id, item_definition) in asset.iter() {
            let entity = item_definition_tracker
                .0
                .entry(item_id.clone())
                .or_insert(commands.spawn_empty().id());

            commands.entity(*entity).insert((
                item_id.clone(),
                item_definition.value.to_owned(),
                ChildOf(*container),
            ));
        }

        item_definition_tracker.0.retain(|item_id, entity| {
            if !asset.contains(item_id) {
                commands.entity(*entity).despawn();
                return false;
            }
            true
        });
    }
}
