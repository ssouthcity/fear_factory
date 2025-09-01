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

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Item(pub Handle<ItemDef>);

// #[derive(Component, Reflect, Default)]
// #[reflect(Component)]
// pub struct Quantity(u32);

// #[derive(Event, Reflect)]
// pub struct PickupItem {
//     pub item: Entity,
//     pub quantity: u32,
// }

// fn pickup_items(
//     trigger: Trigger<PickupItem>,
//     item_defs: Res<Assets<ItemDef>>,
//     items: Query<(&Item, &mut Quantity)>,
// ) {
//     let Ok((item, quantity)) = items.get(trigger.target()) else {
//         return;
//     };
// }

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct PlayerInventory(Inventory);

fn spawn_player_inventory(mut commands: Commands) {
    commands.spawn((
        Name::new("Player Inventory"),
        PlayerInventory(Inventory::sized(9)),
    ));
}
