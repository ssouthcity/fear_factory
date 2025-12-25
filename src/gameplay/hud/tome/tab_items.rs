use bevy::prelude::*;

use crate::{
    gameplay::{
        hud::tome::{TomeTab, UIEntry, UITomeLeftPageRoot},
        item::inventory::Inventory,
        player::Player,
    },
    widgets,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(TomeTab::Items), spawn_item_list);
}

fn spawn_item_list(
    mut commands: Commands,
    left_page: Single<Entity, With<UITomeLeftPageRoot>>,
    q_player: Single<(Entity, &Inventory), With<Player>>,
) {
    let (player, inventory) = *q_player;

    let item_list = commands
        .spawn((
            super::widgets::list_page(),
            ChildOf(*left_page),
            DespawnOnExit(TomeTab::Items),
        ))
        .id();

    for (item_id, _) in inventory.items.iter() {
        commands.spawn((
            widgets::item_plate(player, *item_id),
            UIEntry,
            ChildOf(item_list),
        ));
    }
}
