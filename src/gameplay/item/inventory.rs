use bevy::prelude::*;

use crate::{
    gameplay::{
        item::{assets::ItemDef, stack::Stack},
        storage::StoredBy,
    },
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_inventory);
}

fn spawn_inventory(
    asset_server: Res<AssetServer>,
    item_defs: Res<Assets<ItemDef>>,
    mut commands: Commands,
) {
    let inventory = commands.spawn(Name::new("Inventory")).id();

    for (item_id, item_def) in item_defs.iter() {
        commands.spawn((
            Name::new(item_def.name.clone()),
            Stack {
                item: asset_server.get_id_handle(item_id).unwrap(),
                quantity: 0,
            },
            StoredBy(inventory),
            ChildOf(inventory),
        ));
    }
}
