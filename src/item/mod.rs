use bevy::prelude::*;
use serde::Deserialize;

mod assets;
mod inventory;
mod recipes;
mod stack;

pub use assets::ItemAssets;
pub use inventory::Inventory;
pub use recipes::{Recipe, RecipeAssets, SelectRecipe, SelectedRecipe};
pub use stack::Stack;

pub fn plugin(app: &mut App) {
    app.add_plugins((assets::plugin, recipes::plugin));

    app.register_type::<PlayerInventory>()
        .add_systems(Startup, spawn_player_inventory);
}

#[derive(Debug, Deserialize, TypePath)]
#[allow(dead_code)]
pub struct Item {
    name: String,
    stack_size: u32,
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
