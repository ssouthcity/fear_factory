use bevy::prelude::*;

mod build;
mod deposit;
mod sandbox;

pub use build::{Buildable, Building, QueueSpawnBuilding};
pub use deposit::DepositItem;
pub use sandbox::Sandbox;

use crate::screens::Screen;

pub const SANDBOX_MAP_SIZE: f32 = 1600.0;

pub fn plugin(app: &mut App) {
    app.configure_sets(
        OnEnter(Screen::Gameplay),
        (
            SandboxSpawnSystems::SpawnSandbox,
            SandboxSpawnSystems::SpawnDeposits,
        )
            .chain(),
    );

    app.add_plugins((build::plugin, sandbox::plugin, deposit::plugin));
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SandboxSpawnSystems {
    SpawnSandbox,
    SpawnDeposits,
}
