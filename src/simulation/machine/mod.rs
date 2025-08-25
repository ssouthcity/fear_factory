use bevy::prelude::*;

use crate::assets::manifest::Id;

mod assets;
mod build;
pub mod power;

pub use self::{
    assets::StructureTemplate,
    build::{Preview, QueueStructureSpawn},
};

pub fn plugin(app: &mut App) {
    app.register_type::<Structure>();
    app.register_type::<Machine>();

    app.add_plugins((assets::plugin, build::plugin, power::plugin));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Structure(Id<StructureTemplate>);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Machine;
