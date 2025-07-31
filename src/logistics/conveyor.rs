use bevy::prelude::*;

use crate::{
    FactorySystems,
    logistics::{
        ResourceInput, ResourceOutput,
        io::{ResourceInputInventory, ResourceOutputInventory},
    },
};

pub fn plugin(app: &mut App) {
    app.register_type::<ConveyorBelt>();

    app.add_observer(on_drag_spawn_conveyor_belt);

    app.add_systems(
        Update,
        (
            transfer_belt_contents.in_set(FactorySystems::Logistics),
            draw_conveyor_belts.in_set(FactorySystems::UI),
        ),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ConveyorBelt(Entity, Entity);

fn on_drag_spawn_conveyor_belt(
    trigger: Trigger<Pointer<DragDrop>>,
    resource_inputs: Query<&ResourceInput>,
    resource_outputs: Query<&ResourceOutput>,
    mut commands: Commands,
) {
    let event = trigger.event();

    if event.button != PointerButton::Middle {
        return;
    }

    if !resource_outputs.contains(event.dropped) {
        return;
    }

    if !resource_inputs.contains(event.target) {
        return;
    }

    commands.spawn(ConveyorBelt(event.dropped, event.target));
}

fn transfer_belt_contents(
    conveyor_belts: Query<&ConveyorBelt>,
    mut outputs: Query<&mut ResourceOutputInventory>,
    mut inputs: Query<&mut ResourceInputInventory>,
) {
    for belt in conveyor_belts {
        let Ok(mut output) = outputs.get_mut(belt.0) else {
            continue;
        };

        let Ok(mut input) = inputs.get_mut(belt.1) else {
            continue;
        };

        input.0.add(&output.0);
        output.0.clear();
    }
}

fn draw_conveyor_belts(
    conveyor_belts: Query<&ConveyorBelt>,
    transforms: Query<&Transform>,
    mut gizmos: Gizmos,
) {
    for belt in conveyor_belts {
        let from_position = transforms.get(belt.0).unwrap().translation.truncate();
        let to_position = transforms.get(belt.1).unwrap().translation.truncate();

        gizmos.line_2d(from_position, to_position, Color::hsl(180.0, 1.0, 0.5));
    }
}
