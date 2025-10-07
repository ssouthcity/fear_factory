use bevy::prelude::*;

pub mod assets;
pub mod process;
pub mod progress;
pub mod select;

pub fn plugin(app: &mut App) {
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
pub struct InputOf {
    #[relationship]
    pub entity: Entity,
    pub required_quantity: u32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[relationship_target(relationship = OutputOf, linked_spawn)]
pub struct Outputs(Vec<Entity>);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = Outputs)]
pub struct OutputOf {
    #[relationship]
    pub entity: Entity,
    pub output_quantity: u32,
}
