pub mod dismantle;
pub mod hud;
pub mod item;
pub mod logistics;
pub mod machine;
pub mod power;
pub mod recipe;
pub mod world;

use bevy::prelude::*;

use crate::screens::Screen;

pub fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (
            FactorySystems::Input,
            FactorySystems::Build,
            FactorySystems::Power,
            FactorySystems::Logistics,
            FactorySystems::Work,
            FactorySystems::Dismantle,
            FactorySystems::UI,
        )
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_plugins((
        dismantle::plugin,
        hud::plugin,
        item::plugin,
        logistics::plugin,
        machine::plugin,
        power::plugin,
        recipe::plugin,
        world::plugin,
    ));
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum FactorySystems {
    Input,
    Build,
    Power,
    Logistics,
    Work,
    Dismantle,
    UI,
}
