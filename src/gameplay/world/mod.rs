use bevy::prelude::*;

use crate::screens::Screen;

pub mod construction;
pub mod demolition;
pub mod deposit;
pub mod tilemap;

pub fn plugin(app: &mut App) {
    app.configure_sets(
        OnEnter(Screen::Gameplay),
        (
            WorldSpawnSystems::SpawnMap,
            WorldSpawnSystems::SpawnDeposits,
        )
            .chain(),
    );

    app.add_plugins((
        construction::plugin,
        demolition::plugin,
        deposit::plugin,
        tilemap::plugin,
    ));
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum WorldSpawnSystems {
    SpawnMap,
    SpawnDeposits,
}
