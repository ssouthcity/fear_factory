use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<PowerGrid>();
    app.register_type::<PowerLevel>();
    app.register_type::<PowerGridComponents>();
    app.register_type::<PowerGridComponentOf>();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Name::new("Power Grid"), PowerLevel)]
pub struct PowerGrid;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PowerLevel(pub f32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[relationship_target(relationship = PowerGridComponentOf)]
pub struct PowerGridComponents(Vec<Entity>);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = PowerGridComponents)]
pub struct PowerGridComponentOf(pub Entity);
