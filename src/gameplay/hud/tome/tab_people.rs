use bevy::prelude::*;

use crate::gameplay::{
    hud::tome::{TomeTab, UIEntryList, UITomeLeftPageRoot, widgets},
    people::Person,
    player::Player,
    storage::{Storage, StoredBy},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(TomeTab::People), spawn_people_grid);

    app.add_systems(
        Update,
        (backfill_people_grid, refresh_person_badges).run_if(in_state(TomeTab::People)),
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PersonBadge(pub Entity);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PersonName;

fn person_badge(person: Entity) -> impl Bundle {
    (
        PersonBadge(person),
        Node::default(),
        children![(Text::default(), PersonName,)],
    )
}

fn refresh_person_badges(
    badges: Query<(Entity, &PersonBadge)>,
    children: Query<&Children>,
    people: Query<&Name, With<Person>>,
    mut components: ParamSet<(Query<&mut Text, With<PersonName>>,)>,
) {
    for (badge, PersonBadge(person)) in badges {
        let Ok(name) = people.get(*person) else {
            continue;
        };

        for child in children.iter_descendants(badge) {
            if let Ok(mut text) = components.p0().get_mut(child) {
                text.0 = name.to_string();
            }
        }
    }
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
            widgets::list_page(),
            ChildOf(*left_page),
            DespawnOnExit(TomeTab::People),
        ))
        .id();

    for person in storage
        .iter_descendants(*player)
        .filter(|s| people.contains(*s))
    {
        commands.spawn((person_badge(person), ChildOf(people_grid)));
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

        commands.spawn((person_badge(person), ChildOf(*people_grid)));
    }
}
