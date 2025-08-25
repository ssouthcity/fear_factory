use bevy::prelude::*;

use crate::simulation::item::Inventory;

pub fn plugin(app: &mut App) {
    app.register_type::<InputInventory>();
    app.register_type::<OutputInventory>();
}

#[derive(Component, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
pub struct InputInventory(pub Inventory);

#[derive(Component, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
pub struct OutputInventory(pub Inventory);
