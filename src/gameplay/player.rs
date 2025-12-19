use bevy::prelude::*;

use crate::{
    gameplay::{
        item::{assets::ItemDef, stack::Stack},
        storage::StoredBy,
    },
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_player);
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<ItemDef>>,
) {
    let player = commands.spawn((Name::new("Player"), Player)).id();

    for (item_id, _) in assets.iter() {
        commands.spawn((
            Stack {
                item: asset_server.get_id_handle(item_id).unwrap(),
                quantity: 0,
            },
            ChildOf(player),
            StoredBy(player),
        ));
    }
}
