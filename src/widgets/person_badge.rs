use bevy::prelude::*;

use crate::gameplay::people::Person;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, refresh_person_badges);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct PersonBadge(pub Entity);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct PersonName;

pub fn person_badge(person: Entity) -> impl Bundle {
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
