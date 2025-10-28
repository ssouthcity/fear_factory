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
        FixedUpdate,
        (
            FactorySystems::Construction,
            FactorySystems::Logistics,
            FactorySystems::Work,
            FactorySystems::Demolish,
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
    Construction,
    Logistics,
    Work,
    Demolish,
}
