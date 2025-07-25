use bevy::prelude::*;

use crate::{machine::Machine, power::FuseBroke};

pub fn plugin(app: &mut App) {
    app.register_type::<Powered>();

    app.add_systems(PreUpdate, turn_on_global);

    app.add_observer(on_power_toggle);
    app.add_observer(on_broken_fuse);
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct Powered;

#[derive(Event, Reflect)]
pub struct TogglePower;

fn on_power_toggle(
    trigger: Trigger<TogglePower>,
    powered: Query<&Powered>,
    mut commands: Commands,
) {
    if powered.contains(trigger.target()) {
        commands.entity(trigger.target()).remove::<Powered>();
    } else {
        commands.entity(trigger.target()).insert(Powered);
    }
}

fn on_broken_fuse(
    _trigger: Trigger<FuseBroke>,
    mut commands: Commands,
    powered: Query<Entity, With<Powered>>,
) {
    for entity in powered {
        commands.entity(entity).remove::<Powered>();
    }
}

fn turn_on_global(
    keys: Res<ButtonInput<KeyCode>>,
    machines: Query<Entity, With<Machine>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        for machine in machines {
            commands.entity(machine).insert(Powered);
        }
    }
}
