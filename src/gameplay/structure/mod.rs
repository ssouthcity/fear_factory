use std::time::Duration;

use bevy::prelude::*;

use crate::gameplay::{
    logistics::porter::PorterSpawnTimer, structure::assets::StructureDef,
    world::demolition::Demolishable,
};

pub mod assets;
pub mod default_recipe;
pub mod harvest;
pub mod highlight;
pub mod interactable;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        assets::plugin,
        default_recipe::plugin,
        harvest::plugin,
        highlight::plugin,
        interactable::plugin,
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    PorterSpawnTimer(Timer::new(Duration::from_secs(5), TimerMode::Repeating)),
    Demolishable
)]
pub struct Structure(pub Handle<StructureDef>);
