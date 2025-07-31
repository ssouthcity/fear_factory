use bevy::prelude::*;

use crate::{info::Details, power::grid::GridNode, ui::Highlightable};

pub mod power;
pub mod prefabs;
pub mod work;

pub fn plugin(app: &mut App) {
    app.register_type::<Machine>();

    app.add_plugins((work::plugin, power::plugin));
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Details, GridNode, Highlightable)]
pub struct Machine;
