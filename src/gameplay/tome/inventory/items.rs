use bevy::prelude::*;

use crate::{
    gameplay::{
        inventory::prelude::*,
        player::Player,
        tome::{UITomeLeftPageRoot, inventory::InventoryTabs, list_page},
    },
    widgets,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(InventoryTabs::Items), spawn_item_list);
}

fn spawn_item_list(
    mut commands: Commands,
    left_page: Single<Entity, With<UITomeLeftPageRoot>>,
    player: Single<Entity, With<Player>>,
    inventory: Query<&Inventory>,
) {
    let item_list = commands
        .spawn((
            list_page(),
            ChildOf(*left_page),
            DespawnOnExit(InventoryTabs::Items),
        ))
        .id();

    for slot in inventory.iter_descendants(*player) {
        commands.spawn((widgets::item_plate(slot), ChildOf(item_list)));
    }
}
