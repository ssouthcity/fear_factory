use bevy::prelude::*;

mod assets;
mod build;
mod deposit;
mod terrain;

pub use self::{
    assets::WorldAssets,
    build::{Buildable, Building, QueueSpawnBuilding},
    deposit::DepositRecipe,
    terrain::Terrain,
};

use crate::screens::Screen;

pub const MAP_SIZE: f32 = 1600.0;

pub fn plugin(app: &mut App) {
    app.configure_sets(
        OnEnter(Screen::Gameplay),
        (
            WorldSpawnSystems::SpawnTerrain,
            WorldSpawnSystems::SpawnDeposits,
        )
            .chain(),
    );

    app.add_plugins((
        assets::plugin,
        build::plugin,
        terrain::plugin,
        deposit::plugin,
    ));
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum WorldSpawnSystems {
    SpawnTerrain,
    SpawnDeposits,
}
