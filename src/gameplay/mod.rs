use bevy::prelude::*;

use crate::screens::Screen;

pub mod hud;
pub mod item;
pub mod logistics;
pub mod recipe;
pub mod sprite_sort;
pub mod structure;
pub mod world;

pub fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (
            FactorySystems::Input,
            FactorySystems::Construction,
            FactorySystems::PostConstruction,
            FactorySystems::Harvest,
            FactorySystems::Logistics,
            FactorySystems::Work,
            FactorySystems::Demolish,
            FactorySystems::PostDemolition,
            FactorySystems::UI,
        )
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_plugins((
        hud::plugin,
        item::plugin,
        logistics::plugin,
        recipe::plugin,
        sprite_sort::plugin,
        structure::plugin,
        world::plugin,
    ));
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum FactorySystems {
    Input,
    Construction,
    PostConstruction,
    Harvest,
    Logistics,
    Work,
    Demolish,
    PostDemolition,
    UI,
}
