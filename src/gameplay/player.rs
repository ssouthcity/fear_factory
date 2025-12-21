use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{
    gameplay::{
        item::{assets::ItemDef, stack::Stack},
        people::{Person, naming::NameManager},
        storage::StoredBy,
    },
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_player);

    app.add_systems(
        Update,
        give_player_a_person
            .run_if(in_state(Screen::Gameplay).and(on_timer(Duration::from_secs(5)))),
    );
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

fn give_player_a_person(
    player: Single<Entity, With<Player>>,
    mut commands: Commands,
    mut name_manager: ResMut<NameManager>,
) {
    let name = name_manager.next();

    commands.spawn((
        Name::new(name.clone()),
        Person,
        StoredBy(*player),
        ChildOf(*player),
    ));
}
