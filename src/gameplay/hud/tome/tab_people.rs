use bevy::prelude::*;

use crate::{
    gameplay::{
        hud::tome::{TomeTab, UIEntryList, UITomeLeftPageRoot},
        people::Person,
        player::Player,
        storage::{Storage, StoredBy},
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
    player: Single<Entity, With<Player>>,
    storage: Query<&Storage>,
    people: Query<Entity, With<Person>>,
) {
    let people_grid = commands
        .spawn((
            super::widgets::list_page(),
            ChildOf(*left_page),
            DespawnOnExit(TomeTab::People),
        ))
        .id();

    for person in storage
        .iter_descendants(*player)
        .filter(|s| people.contains(*s))
    {
        commands.spawn((widgets::person_badge(person), ChildOf(people_grid)));
    }
}

fn backfill_people_grid(
    mut commands: Commands,
    people_grid: Single<Entity, With<UIEntryList>>,
    player: Single<Entity, With<Player>>,
    new_people: Query<(Entity, &StoredBy), (With<Person>, Added<StoredBy>)>,
) {
    for (person, stored_by) in new_people {
        if stored_by.0 != *player {
            continue;
        }

        commands.spawn((widgets::person_badge(person), ChildOf(*people_grid)));
    }
}
