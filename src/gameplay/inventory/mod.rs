use bevy::prelude::*;

mod assets;
mod components;
mod messages;
mod prefabs;
mod registry;

#[allow(unused_imports)]
pub mod prelude {
    use super::*;

    pub use assets::ItemDef;
    pub use components::{
        DropOff, InInventory, Input, Inventory, ItemStack, Output, Pickup, Taxonomy, Transport,
    };
    pub use messages::{ItemTransferSystems, TransferItems};
    pub use prefabs::{
        dropoff_slot, empty_slot, input_slot, item_stack_slot, output_slot, pickup_slot,
    };
}

pub fn plugin(app: &mut App) {
    app.add_plugins((assets::plugin,));

    app.add_message::<messages::TransferItems>();

    app.add_systems(
        FixedUpdate,
        messages::transfer_items.in_set(messages::ItemTransferSystems),
    );
}
