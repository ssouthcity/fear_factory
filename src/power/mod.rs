use bevy::prelude::*;

pub mod grid;
pub mod line;
pub mod socket;

pub fn plugin(app: &mut App) {
    app.add_plugins((grid::plugin, line::plugin, socket::plugin));

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
