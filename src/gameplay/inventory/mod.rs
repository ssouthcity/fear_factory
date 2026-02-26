use bevy::prelude::*;

mod assets;
mod components;
mod messages;
mod prefabs;
mod utils;

#[allow(unused_imports)]
pub mod prelude {
    use super::*;

    pub use assets::{ItemDef, ItemTag};
    pub use components::{
        DropOff, InInventory, Input, Inventory, ItemStack, Output, Pickup, Taxonomy, Transport,
    };
    pub use messages::{ItemTransferSystems, TransferItems};
    pub use prefabs::{empty_slot, item_stack_slot};
    pub use utils::{can_afford, spend};
}

pub fn plugin(app: &mut App) {
    app.add_plugins((assets::plugin,));

    app.add_message::<messages::TransferItems>();

    app.add_systems(
        FixedUpdate,
        messages::transfer_items.in_set(messages::ItemTransferSystems),
    );
}
