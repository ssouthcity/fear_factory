use bevy::prelude::*;

pub mod naming;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((naming::plugin,));
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Person;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship_target(relationship = HousedIn, linked_spawn)]
pub struct Houses(Vec<Entity>);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship(relationship_target = Houses)]
pub struct HousedIn(pub Entity);
