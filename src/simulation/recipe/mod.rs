mod assets;
mod process;
mod progress;
mod select;

use bevy::prelude::*;

pub use self::{
    assets::{RecipeDef, RecipeTags},
    process::ProcessState,
    select::{SelectRecipe, SelectedRecipe},
};

pub fn plugin(app: &mut App) {
    app.register_type::<Inputs>();
    app.register_type::<InputOf>();
    app.register_type::<Outputs>();
    app.register_type::<OutputOf>();
    app.register_type::<RequiredQuantity>();

    app.add_plugins((
        assets::plugin,
        process::plugin,
        progress::plugin,
        select::plugin,
    ));
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[relationship_target(relationship = InputOf, linked_spawn)]
pub struct Inputs(Vec<Entity>);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = Inputs)]
pub struct InputOf(pub Entity);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[relationship_target(relationship = OutputOf, linked_spawn)]
pub struct Outputs(Vec<Entity>);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = Outputs)]
pub struct OutputOf(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RequiredQuantity(pub u32);
