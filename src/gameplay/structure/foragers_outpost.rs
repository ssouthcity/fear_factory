use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::gameplay::{
    FactorySystems,
    inventory::prelude::*,
    people::{Assignees, Forager},
    structure::{
        deposit::{Deposit, DepositDef},
        range::Range,
    },
    world::{
        construction::{Constructions, StructureConstructed},
        tilemap::coord::Coord,
    },
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            assign_outpost_taxonomy.run_if(on_message::<StructureConstructed>),
            sync_foragers_outpost_range,
        )
            .chain()
            .in_set(FactorySystems::Forage),
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Inventory)]
pub struct ForagersOutpost;

fn assign_outpost_taxonomy(
    mut structures_constructed: MessageReader<StructureConstructed>,
    mut foragers_outposts: Query<(&Coord, &Range, &mut AseAnimation), With<ForagersOutpost>>,
    deposit_query: Query<&Deposit>,
    deposit_defs: Res<Assets<DepositDef>>,
    constructions: Res<Constructions>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for StructureConstructed(structure) in structures_constructed.read() {
        let Ok((coord, range, mut ase_animation)) = foragers_outposts.get_mut(*structure) else {
            continue;
        };

        let Some(deposit) = range.iter(coord.0).find_map(|pos| {
            constructions
                .get(&pos)
                .and_then(|d| deposit_query.get(*d).ok())
        }) else {
            continue;
        };

        let Some(deposit_def) = deposit_defs.get(&deposit.0) else {
            continue;
        };

        let variant = match deposit_def.taxonomy {
            Taxonomy::Flora => "flora",
            Taxonomy::Fauna => "fauna",
            Taxonomy::Minerale => "minerale",
        };

        ase_animation.aseprite = asset_server.load(format!(
            "sprites/structures/foragers_outpost_{variant}.aseprite"
        ));

        if let Some(handle) = asset_server.get_id_handle(deposit_def.item_id) {
            commands.spawn(pickup_slot(*structure, handle));
        }
    }
}

fn sync_foragers_outpost_range(
    foragers_outposts: Query<(&mut Range, &Assignees), (With<ForagersOutpost>, Changed<Assignees>)>,
    foragers: Query<(), With<Forager>>,
) {
    for (mut range, assignees) in foragers_outposts {
        let foragers = assignees
            .iter()
            .filter(|e| foragers.contains(*e))
            .collect::<Vec<_>>();

        *range = Range::Diamond(foragers.len() as i32);
    }
}
