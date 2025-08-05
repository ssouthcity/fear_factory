use bevy::prelude::*;

pub mod power;
pub mod work;

pub fn plugin(app: &mut App) {
    app.register_type::<Machine>();

    app.add_plugins((work::plugin, power::plugin));
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Machine;
