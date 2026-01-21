use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::Rng;

use crate::{
    gameplay::{
        inventory::prelude::*,
        people::{naming::NameManager, person},
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

fn spawn_player(
    mut commands: Commands,
    item_defs: Res<Assets<ItemDef>>,
    mut seed: ResMut<Seed>,
    asset_server: Res<AssetServer>,
) {
    let player = commands.spawn((Name::new("Player"), Player)).id();

    for (item_id, _) in item_defs.iter() {
        let item_handle = asset_server.get_id_handle(item_id).unwrap();
        let quantity = seed.random_range(0..100);
        commands.spawn(item_stack_slot(player, item_handle, quantity));
    }
}

fn give_player_a_person(mut commands: Commands, mut name_manager: ResMut<NameManager>) {
    commands.spawn(person(&mut name_manager));
}
