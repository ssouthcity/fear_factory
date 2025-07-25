use bevy::prelude::*;

pub mod frequency;
pub mod power;
pub mod prefabs;

pub fn plugin(app: &mut App) {
    app.register_type::<Work>();

    app.add_plugins((frequency::plugin, power::plugin));
}

#[derive(Event, Reflect)]
pub struct Work;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Machine;
