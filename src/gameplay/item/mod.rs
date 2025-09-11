use bevy::prelude::*;

pub mod assets;
pub mod compendium;

pub fn plugin(app: &mut App) {
    app.register_type::<Item>();
    app.register_type::<Quantity>();
    app.register_type::<Full>();

    app.add_systems(Update, mark_full);

    app.add_plugins((assets::plugin, compendium::plugin));
}

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct Item(pub Handle<assets::ItemDef>);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Quantity(pub u32);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct Full;

fn mark_full(
    query: Query<(Entity, &Item, &Quantity)>,
    item_defs: Res<Assets<assets::ItemDef>>,
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
