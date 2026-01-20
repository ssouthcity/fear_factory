use bevy::prelude::*;
use rand::seq::IndexedRandom;

use crate::gameplay::{
    FactorySystems,
    people::porting::{PorterArrival, Porting},
    random::Seed,
    recipe::{assets::Recipe, select::SelectedRecipe},
    world::{
        construction::Constructions,
        tilemap::{CARDINALS, coord::Coord},
    },
};

pub const ARRIVAL_THRESHOLD: f32 = 8.0;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<PathfindingTargetReached>();

    app.add_systems(
        FixedUpdate,
        (move_towards_target, calculate_next_target)
            .chain()
            .in_set(FactorySystems::Logistics),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Walkable;

#[derive(Message)]
struct PathfindingTargetReached(Entity);

fn move_towards_target(
    porters: Query<(Entity, &mut Transform, &mut Sprite, &Porting)>,
    tiles: Query<&Transform, Without<Porting>>,
    mut target_reached: MessageWriter<PathfindingTargetReached>,
    time: Res<Time>,
) {
    for (porter, mut transform, mut sprite, porting) in porters {
        let Ok(target_transform) = tiles.get(porting.target) else {
            continue;
        };

        sprite.flip_x = target_transform.translation.x < transform.translation.x;

        transform.translation = transform.translation.move_towards(
            target_transform.translation,
            porting.speed * time.delta_secs(),
        );

        if transform
            .translation
            .xy()
            .distance(target_transform.translation.xy())
            <= ARRIVAL_THRESHOLD
        {
            target_reached.write(PathfindingTargetReached(porter));
        }
    }
}

fn calculate_next_target(
    mut targets_reached: MessageReader<PathfindingTargetReached>,
    mut porters: Query<&mut Porting>,
    coords: Query<&Coord>,
    walkables: Query<(), With<Walkable>>,
    constructions: Res<Constructions>,
    structures: Query<&SelectedRecipe>,
    recipes: Res<Assets<Recipe>>,
    mut porter_arrived: MessageWriter<PorterArrival>,
    mut seed: ResMut<Seed>,
) {
    for PathfindingTargetReached(porter) in targets_reached.read() {
        let Ok(mut porting) = porters.get_mut(*porter) else {
            continue;
        };

        let target = porting.target;

        porting.visited.insert(target);
        porting.path.push(target);

        let Ok(coord) = coords.get(target) else {
            continue;
        };

        let neighbors: Vec<Entity> = CARDINALS
            .into_iter()
            .map(|c| c + coord.0)
            .filter_map(|c| constructions.get(&c).cloned())
            .collect();

        if let Some(structure) = neighbors.iter().find(|&&entity| {
            let Ok(selected_recipe) = structures.get(entity) else {
                return false;
            };

            let Some(recipe) = recipes.get(&selected_recipe.0) else {
                return false;
            };

            recipe.input.contains_key(&porting.item.id())
        }) {
            porter_arrived.write(PorterArrival {
                porter: *porter,
                destination: *structure,
            });
        }

        let paths: Vec<Entity> = neighbors
            .iter()
            .cloned()
            .filter(|e| {
                walkables.contains(*e) && (porting.backtracking || !porting.visited.contains(e))
            })
            .collect();

        if let Some(t) = paths.choose(&mut seed) {
            porting.target = *t;
            porting.backtracking = false;
        } else if let Some(t) = porting.path.pop() {
            porting.target = t;
            porting.backtracking = true;
        }
    }
}
