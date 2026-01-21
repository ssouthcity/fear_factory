use bevy::{
    input_focus::{InputDispatchPlugin, tab_navigation::TabNavigationPlugin},
    prelude::*,
    ui_widgets::UiWidgetsPlugins,
};

use crate::screens::Screen;

pub mod hud;
pub mod inventory;
pub mod people;
pub mod player;
pub mod random;
pub mod recipe;
pub mod sprite_sort;
pub mod structure;
pub mod tome;
pub mod world;

pub fn plugin(app: &mut App) {
    app.add_plugins((UiWidgetsPlugins, InputDispatchPlugin, TabNavigationPlugin));

    app.configure_sets(
        FixedUpdate,
        (
            FactorySystems::Construction,
            FactorySystems::Work,
            FactorySystems::Demolish,
            people::profession::ProfessionSystems,
            inventory::prelude::ItemTransferSystems,
        )
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_plugins((
        hud::plugin,
        inventory::plugin,
        people::plugin,
        player::plugin,
        random::plugin,
        recipe::plugin,
        sprite_sort::plugin,
        structure::plugin,
        tome::plugin,
        world::plugin,
    ));
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum FactorySystems {
    Construction,
    Work,
    Demolish,
}
