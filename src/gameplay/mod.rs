use bevy::prelude::*;

use crate::screens::Screen;

pub mod hud;
pub mod item;
pub mod logistics;
pub mod recipe;
pub mod structure;
pub mod world;
pub mod y_sort;

pub fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (
            FactorySystems::Input,
            FactorySystems::Build,
            FactorySystems::Power,
            FactorySystems::Logistics,
            FactorySystems::Work,
            FactorySystems::Demolish,
            FactorySystems::UI,
        )
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_plugins((
        hud::plugin,
        item::plugin,
        logistics::plugin,
        structure::plugin,
        recipe::plugin,
        world::plugin,
        y_sort::plugin,
    ));
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum FactorySystems {
    Input,
    Build,
    Power,
    Logistics,
    Work,
    Demolish,
    UI,
}
