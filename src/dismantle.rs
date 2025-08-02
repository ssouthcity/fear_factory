use std::collections::HashSet;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{FactorySystems, build::Building};

const DISMANTLE_BUTTON: KeyCode = KeyCode::KeyF;

pub fn plugin(app: &mut App) {
    app.init_resource::<DismantleTimer>()
        .add_systems(Update, tick_dismantle_timer);

    app.init_resource::<Selection>()
        .init_resource::<DismantleTimer>()
        .add_observer(add_buildings_to_selection)
        .add_observer(remove_buildings_from_selection)
        .add_systems(
            Update,
            (
                clear_selection.run_if(input_just_pressed(KeyCode::Escape)),
                dismantle_selection
                    .run_if(dismantle_timer_held)
                    .in_set(FactorySystems::Build),
            ),
        );
}

#[derive(Resource, Reflect, Default, Deref, DerefMut)]
#[reflect(Resource)]
pub struct Selection(HashSet<Entity>);

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct DismantleTimer(Timer);

impl Default for DismantleTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Once))
    }
}

fn tick_dismantle_timer(
    mut timer: ResMut<DismantleTimer>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if keys.just_released(DISMANTLE_BUTTON) {
        timer.reset();
    }

    if keys.pressed(DISMANTLE_BUTTON) {
        timer.tick(time.delta());
    }
}

fn dismantle_timer_held(timer: Res<DismantleTimer>) -> bool {
    timer.just_finished()
}

fn add_buildings_to_selection(
    trigger: Trigger<Pointer<Over>>,
    mut selection: ResMut<Selection>,
    buildings: Query<Entity, With<Building>>,
) {
    if !buildings.contains(trigger.target) {
        return;
    }

    selection.insert(trigger.target);
}

fn remove_buildings_from_selection(
    trigger: Trigger<Pointer<Out>>,
    mut selection: ResMut<Selection>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.pressed(KeyCode::ShiftLeft) {
        return;
    }

    selection.remove(&trigger.target);
}

fn clear_selection(mut selection: ResMut<Selection>) {
    selection.clear();
}

fn dismantle_selection(mut selection: ResMut<Selection>, mut commands: Commands) {
    for building in selection.iter() {
        commands.entity(*building).despawn();
    }

    selection.clear();
}
