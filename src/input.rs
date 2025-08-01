use std::collections::HashSet;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{FactorySystems, build::Building};

pub fn plugin(app: &mut App) {
    app.init_state::<InputMode>();

    app.init_resource::<Selection>()
        .add_observer(add_buildings_to_selection)
        .add_observer(remove_buildings_from_selection)
        .add_systems(
            Update,
            (
                clear_selection.run_if(input_just_pressed(KeyCode::Escape)),
                dismantle_selection
                    .run_if(input_just_pressed(KeyCode::KeyE).and(in_state(InputMode::Dismantle)))
                    .in_set(FactorySystems::Build),
            ),
        );

    app.add_systems(Update, set_input_mode);
}

#[derive(Resource, Reflect, Default, Deref, DerefMut)]
#[reflect(Resource)]
pub struct Selection(HashSet<Entity>);

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub enum InputMode {
    #[default]
    Normal,
    Dismantle,
}

fn set_input_mode(keys: Res<ButtonInput<KeyCode>>, mut input_mode: ResMut<NextState<InputMode>>) {
    if keys.just_pressed(KeyCode::KeyF) {
        input_mode.set(InputMode::Dismantle);
    }

    if keys.just_pressed(KeyCode::Escape) {
        input_mode.set(InputMode::Normal);
    }
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

fn dismantle_selection(
    mut selection: ResMut<Selection>,
    mut commands: Commands,
    mut input_mode: ResMut<NextState<InputMode>>,
) {
    for building in selection.iter() {
        commands.entity(*building).despawn();
    }

    selection.clear();

    input_mode.set(InputMode::Normal);
}
