use bevy::prelude::*;

mod assets;
mod compendium;
mod inventory;
mod stack;

pub use self::{
    assets::{ItemAssets, ItemDef},
    inventory::Inventory,
    stack::Stack,
};

pub fn plugin(app: &mut App) {
    app.add_plugins((assets::plugin, compendium::plugin));

    app.register_type::<PlayerInventory>()
        .add_systems(Startup, spawn_player_inventory);
}

#[allow(dead_code)]
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Item(Handle<ItemDef>);

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct PlayerInventory(Inventory);

fn spawn_player_inventory(mut commands: Commands) {
    commands.spawn((
        Name::new("Player Inventory"),
        PlayerInventory(Inventory::sized(9)),
    ));
}
