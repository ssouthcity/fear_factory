use bevy::{prelude::*, ui_widgets::RadioButton};

use crate::{
    gameplay::{
        hud::tome::{TomeTab, UIEntry, UITomeLeftPageRoot},
        player::Player,
        storage::ResourceStorage,
    },
    widgets,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(TomeTab::Items), spawn_item_list);
}

fn spawn_item_list(
    mut commands: Commands,
    left_page: Single<Entity, With<UITomeLeftPageRoot>>,
    player: Single<(Entity, &ResourceStorage), With<Player>>,
) {
    let (player, player_storage) = *player;

    let item_list = commands
        .spawn((
            super::widgets::list_page(),
            ChildOf(*left_page),
            DespawnOnExit(TomeTab::Items),
        ))
        .id();

    for resource_id in player_storage.resources.keys() {
        commands.spawn((
            widgets::resource_plate(player, resource_id.clone()),
            RadioButton,
            UIEntry,
            ChildOf(item_list),
        ));
    }
}
