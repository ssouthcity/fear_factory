use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::Rng;

use crate::{
    gameplay::{
        item::{assets::ItemDef, inventory::Inventory},
        people::{Person, naming::NameManager},
        random::Seed,
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

fn spawn_player(mut commands: Commands, item_defs: Res<Assets<ItemDef>>, mut seed: ResMut<Seed>) {
    let mut inventory = Inventory::default();

    for (item_id, _) in item_defs.iter() {
        inventory.items.insert(item_id, seed.random_range(0..100));
    }

    commands.spawn((Name::new("Player"), Player, inventory));
}

fn give_player_a_person(mut commands: Commands, mut name_manager: ResMut<NameManager>) {
    let name = name_manager.next();

    commands.spawn((Name::new(name.clone()), Person));
}
