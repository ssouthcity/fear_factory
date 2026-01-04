use std::time::Duration;

use bevy::prelude::*;

use crate::gameplay::{
    people::porting::PorterCooldown, structure::assets::StructureDef,
    world::demolition::Demolishable,
};

pub mod assets;
pub mod default_recipe;
pub mod harvest;
pub mod highlight;
pub mod interactable;
pub mod path;
pub mod range;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        assets::plugin,
        default_recipe::plugin,
        harvest::plugin,
        highlight::plugin,
        interactable::plugin,
        path::plugin,
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    PorterCooldown(Timer::new(Duration::from_secs(1), TimerMode::Once)),
    Demolishable
)]
pub struct Structure(pub Handle<StructureDef>);
