use bevy::prelude::*;

pub mod assets;
pub mod process;
pub mod progress;
pub mod select;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        assets::plugin,
        process::plugin,
        progress::plugin,
        select::plugin,
    ));
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Input {
    pub quantity: u32,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Output {
    pub quantity: u32,
}
