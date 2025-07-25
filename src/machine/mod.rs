use bevy::prelude::*;

pub mod frequency;

pub fn plugin(app: &mut App) {
    app.register_type::<Work>();

    app.add_plugins((frequency::plugin,));

    app.add_observer(toggle_power);
}

#[derive(Event, Reflect)]
pub struct Work;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct Powered;

#[derive(Event, Reflect)]
pub struct TogglePower;

fn toggle_power(trigger: Trigger<TogglePower>, powered: Query<&Powered>, mut commands: Commands) {
    if powered.contains(trigger.target()) {
        commands.entity(trigger.target()).remove::<Powered>();
    } else {
        commands.entity(trigger.target()).insert(Powered);
    }
}
