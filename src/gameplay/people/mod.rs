use bevy::prelude::*;

pub mod naming;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((naming::plugin,));
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Person;
