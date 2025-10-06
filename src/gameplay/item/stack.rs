use bevy::prelude::*;

use crate::gameplay::item::assets::ItemDef;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, mark_full_stacks);
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Stack {
    pub item: Handle<ItemDef>,
    pub quantity: u32,
}

impl Stack {
    pub fn empty(item: Handle<ItemDef>) -> Stack {
        let quantity = 0;
        Stack { item, quantity }
    }

    pub fn one(item: Handle<ItemDef>) -> Stack {
        let quantity = 1;
        Stack { item, quantity }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct Full;

fn mark_full_stacks(
    query: Query<(Entity, &Stack)>,
    item_defs: Res<Assets<ItemDef>>,
    mut commands: Commands,
) {
    for (entity, stack) in query {
        let stack_size = item_defs
            .get(&stack.item)
            .map(|def| def.stack_size)
            .unwrap_or(1);

        if stack.quantity >= stack_size {
            commands.entity(entity).insert(Full);
        } else {
            commands.entity(entity).remove::<Full>();
        }
    }
}
