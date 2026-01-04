use bevy::prelude::*;

use crate::{
    gameplay::{
        item::inventory::Inventory,
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
    q_player: Single<(Entity, &Inventory), With<Player>>,
) {
    let (player, inventory) = *q_player;

    let item_list = commands
        .spawn((
            list_page(),
            ChildOf(*left_page),
            DespawnOnExit(InventoryTabs::Items),
        ))
        .id();

    for (item_id, _) in inventory.items.iter() {
        commands.spawn((widgets::item_plate(player, *item_id), ChildOf(item_list)));
    }
}
