use bevy::prelude::*;

use crate::{
    info::Details,
    machine::io::{ResourceInput, ResourceInputInventory, ResourceOutput, ResourceOutputInventory},
    power::grid::GridNode,
};

mod io;
pub mod power;
pub mod prefabs;
pub mod work;

pub fn plugin(app: &mut App) {
    app.register_type::<Machine>();

    app.add_plugins((work::plugin, io::plugin, power::plugin));
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Details,
    ResourceInputInventory,
    ResourceOutputInventory,
    ResourceInput,
    ResourceOutput,
    GridNode
)]
pub struct Machine;
