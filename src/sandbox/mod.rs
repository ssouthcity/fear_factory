use bevy::prelude::*;

mod deposit;
mod sandbox;

pub use deposit::Deposit;
pub use sandbox::Sandbox;

pub fn plugin(app: &mut App) {
    app.configure_sets(
        Startup,
        (
            SandboxSpawnSystems::SpawnSandbox,
            SandboxSpawnSystems::SpawnDeposits,
        )
            .chain(),
    );

    app.add_plugins((sandbox::plugin, deposit::plugin));
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SandboxSpawnSystems {
    SpawnSandbox,
    SpawnDeposits,
}
