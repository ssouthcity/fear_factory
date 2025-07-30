use bevy::prelude::*;

pub mod grid;
pub mod pole;

pub fn plugin(app: &mut App) {
    app.add_plugins((grid::plugin, pole::plugin));

    // components
    app.register_type::<PowerProducer>();
    app.register_type::<PowerConsumer>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PowerProducer(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PowerConsumer(pub f32);

#[derive(Event, Reflect)]
pub struct FuseBlown;
