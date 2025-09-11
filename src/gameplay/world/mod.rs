use bevy::prelude::*;

use crate::screens::Screen;

pub mod assets;
pub mod deposit;
pub mod terrain;

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

    app.add_plugins((assets::plugin, terrain::plugin, deposit::plugin));
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum WorldSpawnSystems {
    SpawnTerrain,
    SpawnDeposits,
}
