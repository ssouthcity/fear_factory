pub mod item_slot;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((item_slot::plugin,));
}
