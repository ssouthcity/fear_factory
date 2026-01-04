use bevy::prelude::*;

use crate::{
    gameplay::{
        people::{HousedIn, Person},
        tome::{UITomeLeftPageRoot, inventory::InventoryTabs, list_page},
    },
    widgets,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(InventoryTabs::People), spawn_people_grid);
}

fn spawn_people_grid(
    mut commands: Commands,
    left_page: Single<Entity, With<UITomeLeftPageRoot>>,
    people: Query<Entity, (With<Person>, Without<HousedIn>)>,
) {
    let people_grid = commands
        .spawn((
            list_page(),
            ChildOf(*left_page),
            DespawnOnExit(InventoryTabs::People),
        ))
        .id();

    for person in people {
        commands.spawn((widgets::person_badge(person), ChildOf(people_grid)));
    }
}
