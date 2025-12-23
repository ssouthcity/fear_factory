use std::collections::VecDeque;

use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
};

use crate::gameplay::{
    FactorySystems,
    logistics::{
        path::Pathable,
        porter::{PorterArrival, PorterLost},
    },
    recipe::select::RecipeChanged,
    storage::{InputStorage, OutputStorage},
    world::{
        construction::{Constructions, StructureConstructed},
        tilemap::coord::Coord,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        pathfind
            .in_set(FactorySystems::Logistics)
            .run_if(on_message::<RecipeChanged>.or(on_message::<StructureConstructed>)),
    );

    app.add_systems(
        FixedUpdate,
        walk_along_path.in_set(FactorySystems::Logistics),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WalkPath(pub Vec<Entity>);

fn pathfind(
    structure_query: Query<(Entity, &OutputStorage)>,
    input_query: Query<&InputStorage>,
    pathable_query: Query<&Pathable>,
    coordinates: Query<&Coord>,
    mut commands: Commands,
    constructions: Res<Constructions>,
) {
    for (structure, storage) in structure_query {
        let Some((resource, _)) = storage.resources.iter().next() else {
            continue;
        };

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = HashMap::new();
        let mut solutions = VecDeque::new();

        queue.push_back(structure);
        visited.insert(structure);

        while let Some(current) = queue.pop_front() {
            let coord = coordinates.get(current).unwrap();

            let neighbors: Vec<IVec2> = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
                .into_iter()
                .map(|c| c + coord.0)
                .collect();

            for neighbor_coord in neighbors {
                let Some(neighbor) = constructions.get(&neighbor_coord) else {
                    continue;
                };

                if visited.contains(neighbor) {
                    continue;
                }

                visited.insert(*neighbor);
                parent.insert(neighbor, current);

                let Ok(input_storage) = input_query.get(*neighbor) else {
                    continue;
                };

                if !input_storage.resources.contains_key(resource) {
                    continue;
                }

                let mut path = Vec::new();
                let mut cur = *neighbor;
                while cur != structure {
                    path.push(cur);
                    cur = *parent.get(&cur).unwrap();
                }
                solutions.push_back((*neighbor, path));

                if let Ok(pathable) = pathable_query.get(*neighbor)
                    && pathable.walkable
                {
                    queue.push_back(*neighbor);
                }
            }
        }

        commands.entity(structure).insert(PorterPaths(solutions));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PorterPaths(pub VecDeque<(Entity, Vec<Entity>)>);

fn walk_along_path(
    query: Query<(Entity, &mut Transform, &mut WalkPath, &mut Sprite)>,
    transforms: Query<&Transform, Without<WalkPath>>,
    time: Res<Time>,
    mut porter_arrivals: MessageWriter<PorterArrival>,
    mut porter_losses: MessageWriter<PorterLost>,
) {
    const SPEED: f32 = 64.0;
    const ARRIVAL_THRESHHOLD: f32 = 16.0;

    for (entity, mut transform, mut walk_path, mut sprite) in query {
        let Some(goal) = walk_path.0.last() else {
            continue;
        };

        let Ok(goal_transform) = transforms.get(*goal) else {
            porter_losses.write(PorterLost(entity));
            continue;
        };

        sprite.flip_x = goal_transform.translation.x < transform.translation.x;

        transform.translation = transform
            .translation
            .move_towards(goal_transform.translation, SPEED * time.delta_secs());

        if transform.translation.distance(goal_transform.translation) <= ARRIVAL_THRESHHOLD {
            walk_path.0.pop();

            if walk_path.0.is_empty() {
                porter_arrivals.write(PorterArrival(entity));
            }
        }
    }
}
