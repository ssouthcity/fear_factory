use bevy::prelude::*;

mod assets;
mod conveyor_belt;
mod conveyor_hole;
mod io;

pub use self::{
    assets::LogisticAssets,
    conveyor_hole::{ConveyorHole, ConveyorHoleOf, ConveyorHoles},
    io::{InputInventory, OutputInventory},
};

pub fn plugin(app: &mut App) {
    app.add_plugins((
        assets::plugin,
        conveyor_belt::plugin,
        conveyor_hole::plugin,
        io::plugin,
    ));
}
