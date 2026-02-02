use bevy::{prelude::*, ui_widgets::observe};

use crate::{
    gameplay::{
        people::{
            Assignees, Assignment, Person, Profession,
            profession::{AssignPerson, Forager, UnassignPerson},
        },
        tome::{
            UITomeLeftPageRoot, UITomeRightPageRoot,
            inspect::{InspectTabs, Inspected},
            list_page,
        },
    },
    widgets::{self, person_badge::PersonBadge},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectTabs::ForagerManagement),
        (
            (spawn_forager_list, refresh_forager_list).chain(),
            (spawn_unassigned_list, refresh_unassigned_list).chain(),
        ),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ForagerList;

fn spawn_forager_list(
    mut commands: Commands,
    right_page: Single<Entity, With<UITomeRightPageRoot>>,
) {
    commands.spawn((
        list_page(),
        ForagerList,
        DespawnOnExit(InspectTabs::ForagerManagement),
        ChildOf(*right_page),
        observe(
            |drag_drop: On<Pointer<DragDrop>>,
             inspected: Res<Inspected>,
             badges: Query<&PersonBadge>,
             mut commands: Commands| {
                let Ok(PersonBadge(person)) = badges.get(drag_drop.dropped) else {
                    return;
                };

                commands
                    .entity(drag_drop.dropped)
                    .insert(ChildOf(drag_drop.event_target()));

                commands.trigger(AssignPerson {
                    person: *person,
                    structure: inspected.0,
                    profession: Profession::Forager,
                });
            },
        ),
    ));
}

fn refresh_forager_list(
    mut commands: Commands,
    inspected: Res<Inspected>,
    forager_list: Single<Entity, With<ForagerList>>,
    q_assignees: Query<&Assignees>,
    foragers: Query<(), With<Forager>>,
) {
    commands.entity(*forager_list).despawn_children();

    let Ok(assignees) = q_assignees.get(inspected.0) else {
        return;
    };

    for person in assignees.iter().filter(|e| foragers.contains(*e)) {
        commands.spawn((
            widgets::person_badge(person),
            drag(),
            ChildOf(*forager_list),
        ));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct UnassignedPeopleList;

fn spawn_unassigned_list(
    mut commands: Commands,
    left_page: Single<Entity, With<UITomeLeftPageRoot>>,
) {
    commands.spawn((
        list_page(),
        UnassignedPeopleList,
        DespawnOnExit(InspectTabs::ForagerManagement),
        ChildOf(*left_page),
        observe(
            |drag_drop: On<Pointer<DragDrop>>,
             badges: Query<&PersonBadge>,
             mut commands: Commands| {
                let Ok(PersonBadge(person)) = badges.get(drag_drop.dropped) else {
                    return;
                };

                commands
                    .entity(drag_drop.dropped)
                    .insert(ChildOf(drag_drop.event_target()));

                commands.trigger(UnassignPerson { person: *person });
            },
        ),
    ));
}

fn refresh_unassigned_list(
    mut commands: Commands,
    unassigned_list: Single<Entity, With<UnassignedPeopleList>>,
    people: Query<Entity, (With<Person>, Without<Assignment>)>,
) {
    commands.entity(*unassigned_list).despawn_children();

    for person in people {
        commands.spawn((
            widgets::person_badge(person),
            drag(),
            ChildOf(*unassigned_list),
        ));
    }
}

fn drag() -> impl Bundle {
    (
        UiTransform::default(),
        GlobalZIndex::default(),
        Pickable {
            should_block_lower: false,
            ..default()
        },
        observe(
            |drag: On<Pointer<DragStart>>, mut query: Query<&mut GlobalZIndex>| {
                if let Ok(mut z_index) = query.get_mut(drag.event_target()) {
                    z_index.0 = 1;
                }
            },
        ),
        observe(
            |drag: On<Pointer<Drag>>, mut query: Query<&mut UiTransform>| {
                if let Ok(mut transform) = query.get_mut(drag.event_target()) {
                    transform.translation = Val2::px(drag.distance.x, drag.distance.y);
                }
            },
        ),
        observe(
            |drag: On<Pointer<DragEnd>>,
             mut query: Query<(&mut UiTransform, &mut GlobalZIndex)>| {
                if let Ok((mut transform, mut z_index)) = query.get_mut(drag.event_target()) {
                    transform.translation = Val2::ZERO;
                    z_index.0 = 0;
                }
            },
        ),
    )
}
