use std::{collections::HashMap, time::Duration};

use bevy::prelude::*;

use crate::{
    assets::manifest::{Id, Manifest},
    simulation::{
        item::{Item, manifest::ItemIndex},
        recipe::Recipe,
    },
};

pub fn plugin(app: &mut App) {
    app.register_type::<InputFor>();
    app.register_type::<OutputFor>();
    app.register_type::<Quantity>();

    app.init_resource::<RecipeIndex>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = InputFor)]
pub struct Inputs(Vec<Entity>);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = Inputs)]
pub struct InputFor(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct OutputFor(Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Quantity(u32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TimeToProduce(Duration);

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct RecipeIndex(HashMap<Id<Recipe>, Entity>);

fn spawn_item(
    mut events: EventReader<AssetEvent<Manifest<Recipe>>>,
    assets: Res<Assets<Manifest<Recipe>>>,
    mut recipe_index: ResMut<RecipeIndex>,
    item_index: Res<ItemIndex>,
    mut commands: Commands,
) {
    for event in events.read() {
        if let AssetEvent::Added { id } | AssetEvent::Modified { id } = event {
            let Some(recipe) = assets.get(*id) else {
                continue;
            };

            for (recipe_id, recipe) in recipe.iter() {
                let entity = recipe_index
                    .0
                    .entry(recipe_id.clone())
                    .or_insert_with(|| commands.spawn_empty().id());

                let id = commands
                    .entity(*entity)
                    .insert((
                        recipe_id.clone(),
                        Name::new(recipe.name.to_owned()),
                        TimeToProduce(recipe.duration),
                    ))
                    .id();

                for (item_id, quantity) in recipe.input.iter() {
                    commands.spawn((
                        Name::new("Input"),
                        item_id.clone(),
                        Quantity(*quantity),
                        InputFor(id),
                    ));
                }

                for (item_id, quantity) in recipe.output.iter() {
                    commands.spawn((
                        Name::new("Output"),
                        item_id.clone(),
                        Quantity(*quantity),
                        OutputFor(id),
                    ));
                }
            }
        }
    }
}
