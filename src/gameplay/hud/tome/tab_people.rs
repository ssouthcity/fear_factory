use bevy::prelude::*;

use crate::gameplay::{
    hud::tome::{TomeTab, UITomeLeftPageRoot, widgets},
    people::Person,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(TomeTab::People), spawn_people_grid);

    app.add_systems(
        Update,
        (refresh_person_badges).run_if(in_state(TomeTab::People)),
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
    people: Query<Entity, With<Person>>,
) {
    let people_grid = commands
        .spawn((
            widgets::list_page(),
            ChildOf(*left_page),
            DespawnOnExit(TomeTab::People),
        ))
        .id();

    for person in people {
        commands.spawn((person_badge(person), ChildOf(people_grid)));
    }
}
