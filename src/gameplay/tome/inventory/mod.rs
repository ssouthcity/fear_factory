use bevy::prelude::*;

use crate::gameplay::tome::{TomeMenu, tome_plugin::TomePlugin};

pub mod items;
pub mod people;
pub mod recipes;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TomePlugin::<InventoryTabs> {
        menu: TomeMenu::Inventory,
        tabs: vec![
            ("Items", InventoryTabs::Items),
            ("People", InventoryTabs::People),
            ("Recipes", InventoryTabs::Recipes),
        ],
    });

    app.add_plugins((items::plugin, people::plugin, recipes::plugin));
}

#[derive(SubStates, Component, Reflect, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
#[source(TomeMenu = TomeMenu::Inventory)]
#[reflect(Component)]
pub enum InventoryTabs {
    #[default]
    Items,
    People,
    Recipes,
}
