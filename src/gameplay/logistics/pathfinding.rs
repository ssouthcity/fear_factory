use std::collections::VecDeque;

use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
};

use crate::gameplay::{
    FactorySystems,
    item::Item,
    logistics::{
        path::{Path, Pathable, Paths},
        porter::{PorterArrival, PorterOf, PorterToInput},
    },
    recipe::{InputOf, Inputs},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<WalkPath>();

    app.add_observer(pathfind_for_porter);

    app.add_systems(Update, walk_along_path.in_set(FactorySystems::Logistics));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WalkPath(Vec<Entity>);

fn pathfind_for_porter(
    trigger: Trigger<OnAdd, PorterOf>,
    porter_query: Query<(&Item, &PorterOf)>,
    inputs_query: Query<&Inputs>,
    input_query: Query<&Item, With<InputOf>>,
    nodes: Query<(&Pathable, &Paths)>,
    edges: Query<&Path>,
    mut commands: Commands,
) {
    let (held_item, PorterOf(start)) = porter_query.get(trigger.target()).unwrap();

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();

    queue.push_back(*start);
    visited.insert(*start);

    while let Some(current) = queue.pop_front() {
        let (_pathable, paths) = nodes.get(current).unwrap();

        // if !pathable.walkable {
        //     continue;
        // }

        for path in paths.0.iter() {
            let edge = edges.get(*path).unwrap();
            let neighbor = edge.other(current);

            if visited.contains(&neighbor) {
                continue;
            }

            visited.insert(neighbor);
            parent.insert(neighbor, current);

            let input_entity = inputs_query.get(neighbor).ok().and_then(|inputs| {
                inputs.iter().find(|input| {
                    input_query
                        .get(*input)
                        .is_ok_and(|item| item.0 == held_item.0)
                })
            });

            if let Some(goal) = input_entity {
                let mut path = Vec::new();
                let mut cur = neighbor.clone();
                while cur != *start {
                    path.push(cur);
                    cur = *parent.get(&cur).unwrap();
                }
                commands
                    .entity(trigger.target())
                    .insert((PorterToInput(goal), WalkPath(path)));
                return;
            }

            queue.push_back(neighbor);
        }
    }
}

fn walk_along_path(
    query: Query<(Entity, &mut Transform, &mut WalkPath)>,
    transforms: Query<&Transform, Without<WalkPath>>,
    time: Res<Time>,
    mut events: EventWriter<PorterArrival>,
) {
    const SPEED: f32 = 32.0;
    const ARRIVAL_THRESHHOLD: f32 = 32.0;

    for (entity, mut transform, mut walk_path) in query {
        let Some(goal) = walk_path.0.last() else {
            continue;
        };

        let Ok(goal_transform) = transforms.get(*goal) else {
            continue;
        };

        transform.translation = transform
            .translation
            .move_towards(goal_transform.translation, SPEED * time.delta_secs());

        if transform.translation.distance(goal_transform.translation) <= ARRIVAL_THRESHHOLD {
            walk_path.0.pop();

            if walk_path.0.len() == 0 {
                events.write(PorterArrival(entity));
            }
        }
    }
}
