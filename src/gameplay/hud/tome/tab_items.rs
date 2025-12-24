use bevy::prelude::*;

use crate::{
    gameplay::{
        hud::tome::{TomeTab, UIEntry, UITomeLeftPageRoot},
        item::stack::Stack,
        player::Player,
        storage::Storage,
    },
    widgets,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(TomeTab::Items), spawn_item_list);
}

fn spawn_item_list(
    mut commands: Commands,
    left_page: Single<Entity, With<UITomeLeftPageRoot>>,
    q_player: Single<&Storage, With<Player>>,
    q_items: Query<Entity, With<Stack>>,
) {
    let item_list = commands
        .spawn((
            super::widgets::list_page(),
            ChildOf(*left_page),
            DespawnOnExit(TomeTab::Items),
        ))
        .id();

    for stack in q_player.iter().flat_map(|e| q_items.get(e)) {
        commands.spawn((widgets::item_plate(stack), UIEntry, ChildOf(item_list)));
    }
}
