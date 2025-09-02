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
    app.register_type::<Item>();
    app.register_type::<Quantity>();
    app.register_type::<Full>();

    app.add_systems(Update, mark_full);

    app.add_plugins((assets::plugin, compendium::plugin));

    app.register_type::<PlayerInventory>()
        .add_systems(Startup, spawn_player_inventory);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Item(pub Handle<ItemDef>);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Quantity(pub u32);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct Full;

fn mark_full(
    query: Query<(Entity, &Item, &Quantity)>,
    item_defs: Res<Assets<ItemDef>>,
    mut commands: Commands,
) {
    for (entity, item, quantity) in query {
        let stack_size = item_defs
            .get(&item.0)
            .map(|def| def.stack_size)
            .unwrap_or(1);

        if quantity.0 >= stack_size {
            commands.entity(entity).insert(Full);
        } else {
            commands.entity(entity).remove::<Full>();
        }
    }
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
