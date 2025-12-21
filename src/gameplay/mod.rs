use bevy::prelude::*;

use crate::screens::Screen;

pub mod hud;
pub mod item;
pub mod logistics;
pub mod people;
pub mod player;
pub mod random;
pub mod recipe;
pub mod sprite_sort;
pub mod storage;
pub mod structure;
pub mod world;

pub fn plugin(app: &mut App) {
    app.configure_sets(
        FixedUpdate,
        (
            FactorySystems::Construction,
            FactorySystems::Harvest,
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
        people::plugin,
        player::plugin,
        random::plugin,
        recipe::plugin,
        sprite_sort::plugin,
        structure::plugin,
        world::plugin,
    ));
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum FactorySystems {
    Construction,
    Harvest,
    Logistics,
    Work,
    Demolish,
}
