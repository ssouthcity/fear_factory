use std::collections::HashSet;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{simulation::FactorySystems, simulation::machine::Structure};

const DISMANTLE_BUTTON: KeyCode = KeyCode::KeyF;

pub fn plugin(app: &mut App) {
    app.register_type::<QueueDismantle>();
    app.register_type::<Selection>();
    app.register_type::<DismantleTimer>();

    app.add_event::<QueueDismantle>();

    app.init_resource::<DismantleTimer>()
        .add_systems(Update, tick_dismantle_timer);

    app.init_resource::<Selection>()
        .add_observer(add_buildings_to_selection)
        .add_observer(remove_buildings_from_selection);

    app.add_systems(
        Update,
        (
            clear_selection.run_if(input_just_pressed(KeyCode::Escape)),
            queue_dismantle_selection.run_if(dismantle_timer_held),
        ),
    );

    app.add_systems(
        Update,
        dismantle_buildings.in_set(FactorySystems::Dismantle),
    );
}

#[derive(Event, Reflect)]
pub struct QueueDismantle(pub Entity);

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
    buildings: Query<Entity, With<Structure>>,
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

fn queue_dismantle_selection(
    mut selection: ResMut<Selection>,
    mut events: EventWriter<QueueDismantle>,
) {
    for building in selection.iter() {
        events.write(QueueDismantle(*building));
    }

    selection.clear();
}

fn dismantle_buildings(mut events: EventReader<QueueDismantle>, mut commands: Commands) {
    for event in events.read() {
        commands.entity(event.0).despawn();
    }
}
