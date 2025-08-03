use bevy::prelude::*;

mod build;
mod deposit;
mod sandbox;

pub use build::{Buildable, Building, QueueSpawnBuilding};
pub use deposit::Deposit;
pub use sandbox::Sandbox;

pub const SANDBOX_MAP_SIZE: f32 = 1600.0;
pub const COAL_DEPOSITS: u8 = 4;
pub const IRON_DEPOSITS: u8 = 8;

pub fn plugin(app: &mut App) {
    app.configure_sets(
        Startup,
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
