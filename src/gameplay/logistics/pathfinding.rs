use std::collections::VecDeque;

use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
};

use crate::gameplay::{
    FactorySystems,
    item::Item,
    logistics::{
        path::{Pathable, PathsUpdated},
        porter::{PorterArrival, PorterLost},
    },
    recipe::{InputOf, Inputs, OutputOf, select::RecipeChanged},
    world::{construction::Constructions, tilemap::coord::Coord},
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
    pathable_query: Query<&Pathable>,
    coordinates: Query<&Coord>,
    mut commands: Commands,
    constructions: Res<Constructions>,
) {
    for (entity, item, OutputOf(start)) in output_query {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = HashMap::new();
        let mut solutions = VecDeque::new();

        queue.push_back(*start);
        visited.insert(*start);

        while let Some(current) = queue.pop_front() {
            let coord = coordinates.get(current).unwrap();

            let neighbors = vec![
                UVec2::new(coord.x.saturating_add(1), coord.y),
                UVec2::new(coord.x.saturating_sub(1), coord.y),
                UVec2::new(coord.x, coord.y.saturating_add(1)),
                UVec2::new(coord.x, coord.y.saturating_sub(1)),
            ];

            for neighbor_coord in neighbors {
                let Some(neighbor) = constructions.get(&neighbor_coord) else {
                    continue;
                };

                if visited.contains(neighbor) {
                    continue;
                }

                visited.insert(*neighbor);
                parent.insert(neighbor, current);

                let input_entity = inputs_query.get(*neighbor).ok().and_then(|inputs| {
                    inputs
                        .iter()
                        .find(|input| input_query.get(*input).is_ok_and(|i| i.0 == item.0))
                });

                if let Some(goal) = input_entity {
                    let mut path = Vec::new();
                    let mut cur = *neighbor;
                    while cur != *start {
                        path.push(cur);
                        cur = *parent.get(&cur).unwrap();
                    }
                    solutions.push_back((goal, path));
                }

                if let Ok(pathable) = pathable_query.get(*neighbor)
                    && pathable.walkable
                {
                    queue.push_back(*neighbor);
                }
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
    mut lost_events: EventWriter<PorterLost>,
) {
    const SPEED: f32 = 64.0;
    const ARRIVAL_THRESHHOLD: f32 = 8.0;

    for (entity, mut transform, mut walk_path) in query {
        let Some(goal) = walk_path.0.last() else {
            continue;
        };

        let Ok(goal_transform) = transforms.get(*goal) else {
            lost_events.write(PorterLost(entity));
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
