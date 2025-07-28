use bevy::prelude::*;

use crate::{
    input::InputMode,
    power::{
        grid::{MergeGrids, PowerGridComponentOf},
        pole::PowerPole,
    },
};

pub fn plugin(app: &mut App) {
    app.init_resource::<SelectedPowerPole>();

    app.add_observer(on_add_power_pole);
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct SelectedPowerPole(Option<Entity>);

fn on_add_power_pole(trigger: Trigger<OnAdd, PowerPole>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .observe(on_power_line_select);
}

fn on_power_line_select(
    trigger: Trigger<Pointer<Click>>,
    power_poles: Query<Entity, With<PowerPole>>,
    mut selected_pole: ResMut<SelectedPowerPole>,
    state: Res<State<InputMode>>,
    mut merge_grid_events: EventWriter<MergeGrids>,
    grids: Query<&PowerGridComponentOf>,
) {
    if *state.get() != InputMode::PowerLine {
        return;
    }

    let event = trigger.event();

    if event.button != PointerButton::Primary {
        selected_pole.0 = None;
        return;
    }

    let Ok(pole) = power_poles.get(trigger.target()) else {
        unreachable!("observer must be attatched to a power pole");
    };

    if let Some(other) = selected_pole.0 {
        let this_grid = grids.get(trigger.target()).unwrap();
        let other_grid = grids.get(other).unwrap();

        merge_grid_events.write(MergeGrids(this_grid.0, other_grid.0));
        selected_pole.0 = None;
    } else {
        selected_pole.0 = Some(pole);
    }
}
