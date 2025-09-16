use std::collections::VecDeque;

use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
};

use crate::gameplay::{
    FactorySystems,
    item::Item,
    logistics::{
        path::{Path, Pathable, Paths, PathsUpdated},
        porter::PorterArrival,
    },
    recipe::{InputOf, Inputs, OutputOf, select::RecipeChanged},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<WalkPath>();
    app.register_type::<PorterPaths>();

    app.add_systems(
        Update,
        pathfind
            .in_set(FactorySystems::Logistics)
            .run_if(on_event::<RecipeChanged>.or(on_event::<PathsUpdated>)),
    );

    app.add_systems(Update, walk_along_path.in_set(FactorySystems::Logistics));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WalkPath(pub Vec<Entity>);

fn pathfind(
    output_query: Query<(Entity, &Item, &OutputOf)>,
    inputs_query: Query<&Inputs>,
    input_query: Query<&Item, With<InputOf>>,
    nodes: Query<(&Pathable, &Paths)>,
    edges: Query<&Path>,
    mut commands: Commands,
) {
    for (entity, item, OutputOf(start)) in output_query {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = HashMap::new();
        let mut solutions = VecDeque::new();

        queue.push_back(*start);
        visited.insert(*start);

        while let Some(current) = queue.pop_front() {
            let (_, paths) = nodes.get(current).unwrap();

            for path in paths.0.iter() {
                let edge = edges.get(*path).unwrap();
                let neighbor = edge.other(current);

                if visited.contains(&neighbor) {
                    continue;
                }

                visited.insert(neighbor);
                parent.insert(neighbor, current);

                let input_entity = inputs_query.get(neighbor).ok().and_then(|inputs| {
                    inputs
                        .iter()
                        .find(|input| input_query.get(*input).is_ok_and(|i| i.0 == item.0))
                });

                if let Some(goal) = input_entity {
                    let mut path = Vec::new();
                    let mut cur = neighbor;
                    while cur != *start {
                        path.push(cur);
                        cur = *parent.get(&cur).unwrap();
                    }
                    solutions.push_back((goal, path));
                }

                let (pathable, _) = nodes.get(neighbor).unwrap();
                if !pathable.walkable {
                    continue;
                }

                queue.push_back(neighbor);
            }
        }

        commands.entity(entity).insert(PorterPaths(solutions));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PorterPaths(pub VecDeque<(Entity, Vec<Entity>)>);

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

            if walk_path.0.is_empty() {
                events.write(PorterArrival(entity));
            }
        }
    }
}
