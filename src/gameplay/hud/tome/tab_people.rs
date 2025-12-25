use bevy::prelude::*;

use crate::{
    gameplay::{
        hud::tome::{TomeTab, UIEntryList, UITomeLeftPageRoot},
        people::{HousedIn, Person},
    },
    widgets,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(TomeTab::People), spawn_people_grid);

    app.add_systems(
        Update,
        backfill_people_grid.run_if(in_state(TomeTab::People)),
    );
}

fn spawn_people_grid(
    mut commands: Commands,
    left_page: Single<Entity, With<UITomeLeftPageRoot>>,
    people: Query<Entity, (With<Person>, Without<HousedIn>)>,
) {
    let people_grid = commands
        .spawn((
            super::widgets::list_page(),
            ChildOf(*left_page),
            DespawnOnExit(TomeTab::People),
        ))
        .id();

    for person in people {
        commands.spawn((widgets::person_badge(person), ChildOf(people_grid)));
    }
}

fn backfill_people_grid(
    mut commands: Commands,
    people_grid: Single<Entity, With<UIEntryList>>,
    new_people: Query<Entity, (Added<Person>, Without<HousedIn>)>,
) {
    for person in new_people {
        commands.spawn((widgets::person_badge(person), ChildOf(*people_grid)));
    }
}
