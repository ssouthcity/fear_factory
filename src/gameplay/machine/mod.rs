use bevy::prelude::*;

pub mod assets;
pub mod build;

pub fn plugin(app: &mut App) {
    app.register_type::<Structure>();
    app.register_type::<Machine>();

    app.add_plugins((assets::plugin, build::plugin));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Structure;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Machine;
