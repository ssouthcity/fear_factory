use std::collections::HashSet;

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::gameplay::{
    FactorySystems,
    item::assets::Taxonomy,
    world::{
        construction::{Constructions, StructureConstructed},
        deposit::Deposit,
        tilemap::coord::Coord,
    },
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            assign_harvester_taxonomy.run_if(on_message::<StructureConstructed>),
            assign_harvested_deposits,
            harvest_deposit,
        )
            .chain()
            .in_set(FactorySystems::Harvest),
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Harvester;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct HarvestsFrom(pub HashSet<Entity>);

fn assign_harvester_taxonomy(
    mut structures_constructed: MessageReader<StructureConstructed>,
    harvester_query: Query<(&Coord, &Range), With<Harvester>>,
    deposit_query: Query<&Taxonomy, With<Deposit>>,
    constructions: Res<Constructions>,
    mut commands: Commands,
) {
    for StructureConstructed(structure) in structures_constructed.read() {
        let Ok((coord, range)) = harvester_query.get(*structure) else {
            continue;
        };

        let Some(taxonomy) = range.iter(coord.0).find_map(|pos| {
            constructions
                .get(&pos)
                .and_then(|d| deposit_query.get(*d).ok())
        }) else {
            continue;
        };

        commands.entity(*structure).insert(taxonomy.clone());
    }
}

fn assign_harvested_deposits(
    harvester_query: Query<
        (Entity, &Coord, &Taxonomy, &Range, &mut AseAnimation),
        (With<Harvester>, Changed<Taxonomy>),
    >,
    deposit_query: Query<(Entity, &Taxonomy), With<Deposit>>,
    constructions: Res<Constructions>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (harvester, coord, taxonomy, range, mut ase_animation) in harvester_query {
        let deposits: HashSet<Entity> = range
            .iter(coord.0)
            .filter_map(|pos| {
                constructions
                    .get(&pos)
                    .and_then(|construction| deposit_query.get(*construction).ok())
            })
            .filter(|(_, deposit_taxonomy)| *deposit_taxonomy == taxonomy)
            .map(|(entity, _)| entity)
            .collect();

        let variant = match taxonomy {
            Taxonomy::Flora => "flora",
            Taxonomy::Fauna => "fauna",
            Taxonomy::Minerale => "minerale",
        };

        ase_animation.aseprite =
            asset_server.load(format!("sprites/structures/harvester_{}.aseprite", variant));

        commands.entity(harvester).insert(HarvestsFrom(deposits));
    }
}

fn harvest_deposit(
    harvester_query: Query<&mut HarvestsFrom, With<Harvester>>,
    mut deposit_query: Query<(&mut Deposit, &Coord)>,
    mut commands: Commands,
    mut constructions: ResMut<Constructions>,
) {
    for mut harvests_from in harvester_query {
        let Some(deposit) = harvests_from.0.iter().next().cloned() else {
            continue;
        };

        let Ok((mut deposit_data, coord)) = deposit_query.get_mut(deposit) else {
            harvests_from.0.remove(&deposit);
            continue;
        };

        deposit_data.quantity = deposit_data.quantity.saturating_sub(1);

        if deposit_data.quantity == 0 {
            commands.entity(deposit).despawn();
            constructions.remove(coord);
            harvests_from.0.remove(&deposit);
        }
    }
}

/// A range shape used for area targeting or effect zones.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub enum Range {
    /// A diamond pattern with the given radius. Every point in the pattern has a manhattan distance
    /// that is equal to or less than said radius
    Diamond(i32),
}

impl Range {
    fn pattern_diamond(position: IVec2, range: i32) -> impl Iterator<Item = IVec2> {
        (-range..=range).flat_map(move |dy| {
            let max_dx = range - dy.abs();
            (-max_dx..=max_dx).filter_map(move |dx| {
                let pos = IVec2::new(position.x - dx, position.y - dy);
                (pos != position).then_some(pos)
            })
        })
    }

    /// Returns an iterator that loops over all the points in the range
    pub fn iter(&self, position: IVec2) -> impl Iterator<Item = IVec2> {
        match self {
            Self::Diamond(range) => Self::pattern_diamond(position, *range),
        }
    }

    /// Checks whether a point is contained within anothers radius
    pub fn contains(&self, position: &IVec2, other: &IVec2) -> bool {
        match self {
            Self::Diamond(range) => position.manhattan_distance(*other) <= *range as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_diamond_range_1_iter() {
        let range = Range::Diamond(1);
        let position = IVec2::new(0, 0);

        let result: HashSet<_> = range.iter(position).collect();

        // Expected positions (Manhattan distance <= 1, excluding origin)
        let expected: HashSet<_> = [
            IVec2::new(0, 1),
            IVec2::new(1, 0),
            IVec2::new(0, -1),
            IVec2::new(-1, 0),
        ]
        .into_iter()
        .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_diamond_range_2_iter() {
        let range = Range::Diamond(2);
        let position = IVec2::new(0, 0);

        let result: Vec<_> = range.iter(position).collect();

        // Diamond of radius 2 has 12 tiles excluding center
        assert_eq!(result.len(), 12);

        // Some known included points
        assert!(result.contains(&IVec2::new(0, 2)));
        assert!(result.contains(&IVec2::new(2, 0)));
        assert!(result.contains(&IVec2::new(-1, 1)));

        // Known excluded point (outside diamond)
        assert!(!result.contains(&IVec2::new(2, 2)));
    }

    #[test]
    fn test_contains_true_cases() {
        let range = Range::Diamond(2);
        let origin = IVec2::new(0, 0);

        let inside_points = [
            IVec2::new(1, 1),
            IVec2::new(2, 0),
            IVec2::new(0, -2),
            IVec2::new(-1, 1),
        ];

        for p in inside_points {
            assert!(
                range.contains(&origin, &p),
                "Expected {:?} to be inside diamond of range 2",
                p
            );
        }
    }

    #[test]
    fn test_contains_false_cases() {
        let range = Range::Diamond(2);
        let origin = IVec2::new(0, 0);

        let outside_points = [IVec2::new(2, 1), IVec2::new(3, 0), IVec2::new(-2, -2)];

        for p in outside_points {
            assert!(
                !range.contains(&origin, &p),
                "Expected {:?} to be outside diamond of range 2",
                p
            );
        }
    }

    #[test]
    fn test_iter_excludes_origin() {
        let range = Range::Diamond(3);
        let origin = IVec2::new(5, 5);

        let result: Vec<_> = range.iter(origin).collect();

        assert!(
            !result.contains(&origin),
            "Iterator should not include the origin position itself"
        );
    }

    #[test]
    fn test_iter_positions_match_contains() {
        // Sanity check: all iterator positions must satisfy `contains`
        let range = Range::Diamond(3);
        let origin = IVec2::new(0, 0);

        for pos in range.iter(origin) {
            assert!(
                range.contains(&origin, &pos),
                "Iterator yielded {:?}, but contains() returned false",
                pos
            );
        }
    }
}
