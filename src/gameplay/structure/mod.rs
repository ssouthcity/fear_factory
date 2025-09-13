use bevy::prelude::*;

use crate::gameplay::logistics::path::Pathable;

pub mod assets;
pub mod build;
pub mod dismantle;
pub mod highlight;
pub mod interactable;

pub fn plugin(app: &mut App) {
    app.register_type::<Structure>();

    app.add_plugins((
        assets::plugin,
        build::plugin,
        dismantle::plugin,
        highlight::plugin,
        interactable::plugin,
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(Pathable)]
pub struct Structure;
