use std::collections::VecDeque;

use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
};

use crate::gameplay::{
    FactorySystems,
    people::porting::{PorterArrival, PorterLost, Porting},
    recipe::{
        assets::Recipe,
        select::{RecipeChanged, SelectedRecipe},
    },
    world::{
        construction::{Constructions, StructureConstructed},
        demolition::Demolished,
        tilemap::coord::Coord,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        pathfind.in_set(FactorySystems::Logistics).run_if(
            on_message::<RecipeChanged>
                .or(on_message::<StructureConstructed>.or(on_message::<Demolished>)),
        ),
    );

    app.add_systems(
        FixedUpdate,
        walk_along_path.in_set(FactorySystems::Logistics),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Pathable {
    pub walkable: bool,
}

impl Pathable {
    pub fn walkable() -> Self {
        Self { walkable: true }
    }
}

fn pathfind(
    structures: Query<(Entity, &SelectedRecipe)>,
    recipes: Res<Assets<Recipe>>,
    pathable_query: Query<&Pathable>,
    coordinates: Query<&Coord>,
    mut commands: Commands,
    constructions: Res<Constructions>,
) {
    for (structure, selected_recipe) in structures {
        let Some(recipe) = recipes.get(&selected_recipe.0) else {
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

                if let Ok(pathable) = pathable_query.get(*neighbor)
                    && pathable.walkable
                {
                    queue.push_back(*neighbor);
                }

                let Ok((_, other_selected_recipe)) = structures.get(*neighbor) else {
                    continue;
                };

                let Some(other_recipe) = recipes.get(&other_selected_recipe.0) else {
                    continue;
                };

                let is_goal = recipe
                    .output
                    .iter()
                    .any(|output| other_recipe.input.contains_key(output.0));

                if is_goal {
                    let mut path = Vec::new();
                    let mut cur = *neighbor;
                    while cur != structure {
                        path.push(cur);
                        cur = *parent.get(&cur).unwrap();
                    }
                    solutions.push_back((*neighbor, path));
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
    porters: Query<(Entity, &mut Transform, &mut Porting, &mut Sprite)>,
    transforms: Query<&Transform, Without<Porting>>,
    time: Res<Time>,
    mut porter_arrivals: MessageWriter<PorterArrival>,
    mut porter_losses: MessageWriter<PorterLost>,
) {
    const SPEED: f32 = 64.0;
    const ARRIVAL_THRESHHOLD: f32 = 16.0;

    for (entity, mut transform, mut porting, mut sprite) in porters {
        let Some(goal) = porting.path.last() else {
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
            porting.path.pop();

            if porting.path.is_empty() {
                porter_arrivals.write(PorterArrival(entity));
            }
        }
    }
}
