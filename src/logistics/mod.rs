use bevy::prelude::*;

mod conveyor_belt;
mod conveyor_hole;
mod io;

pub use conveyor_hole::{ConveyorHole, ConveyorHoleOf, ConveyorHoles};
pub use io::InputFilter;

pub fn plugin(app: &mut App) {
    app.add_plugins((conveyor_belt::plugin, conveyor_hole::plugin, io::plugin));
}
