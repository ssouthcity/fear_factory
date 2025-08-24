use bevy::prelude::*;

mod assets;
mod inventory;
mod manifest;
mod stack;

pub use self::{
    assets::ItemAssets,
    inventory::Inventory,
    manifest::{Item, StackSize},
    stack::Stack,
};

pub fn plugin(app: &mut App) {
    app.add_plugins((assets::plugin, manifest::plugin));

    app.register_type::<PlayerInventory>()
        .add_systems(Startup, spawn_player_inventory);
}

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct PlayerInventory(Inventory);

fn spawn_player_inventory(mut commands: Commands) {
    commands.spawn((
        Name::new("Player Inventory"),
        PlayerInventory(Inventory::sized(9)),
    ));
}
