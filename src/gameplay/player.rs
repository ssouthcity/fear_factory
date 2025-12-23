use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{
    gameplay::{
        item::assets::ItemDef,
        people::{Person, naming::NameManager},
        storage::{ResourceID, ResourceStorage},
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

fn spawn_player(mut commands: Commands, assets: Res<Assets<ItemDef>>) {
    let mut storage = ResourceStorage::default();
    for (_, item_def) in assets.iter() {
        storage.resources.insert(ResourceID(item_def.id.clone()), 0);
    }

    commands.spawn((Name::new("Player"), Player, storage));
}

fn give_player_a_person(
    player: Single<Entity, With<Player>>,
    mut commands: Commands,
    mut name_manager: ResMut<NameManager>,
) {
    let name = name_manager.next();

    commands.spawn((Name::new(name.clone()), Person, ChildOf(*player)));
}
