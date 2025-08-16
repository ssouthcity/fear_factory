use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSlice;

use crate::{
    FactorySystems,
    logistics::{LogisticAssets, conveyor_belt::QueueConveyorSpawn},
};

pub fn plugin(app: &mut App) {
    app.register_type::<ConveyorHole>();
    app.register_type::<ConveyorHoles>();
    app.register_type::<ConveyorHoleOf>();

    app.add_observer(on_drag_drop_conveyor_holes);

    app.add_systems(
        Update,
        (add_child_of_to_conveyor_hole, add_sprite_to_conveyor_hole).in_set(FactorySystems::UI),
    );
}

#[derive(Component, Reflect, Default, PartialEq, Eq)]
#[reflect(Component)]
#[require(Pickable)]
pub enum ConveyorHole {
    #[default]
    Outbound,
    Inbound,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = ConveyorHoleOf, linked_spawn)]
pub struct ConveyorHoles(Vec<Entity>);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = ConveyorHoles)]
pub struct ConveyorHoleOf(pub Entity);

fn add_child_of_to_conveyor_hole(
    query: Query<(Entity, &ConveyorHoleOf), Added<ConveyorHoleOf>>,
    mut commands: Commands,
) {
    for (entity, conveyor_hole_of) in query {
        commands.entity(entity).insert(ChildOf(conveyor_hole_of.0));
    }
}

fn add_sprite_to_conveyor_hole(
    query: Query<(Entity, &Transform), Added<ConveyorHole>>,
    logistic_assets: Res<LogisticAssets>,
    mut commands: Commands,
) {
    for (entity, transform) in query {
        commands.entity(entity).insert((
            Sprite::sized(Vec2::splat(24.0)),
            AseSlice {
                aseprite: logistic_assets.conveyor_holes.clone(),
                name: if transform.translation.x >= 0.0 {
                    "right".to_string()
                } else {
                    "left".to_string()
                },
            },
        ));
    }
}

fn on_drag_drop_conveyor_holes(
    mut trigger: Trigger<Pointer<DragDrop>>,
    conveyor_holes: Query<&ConveyorHole>,
    mut events: EventWriter<QueueConveyorSpawn>,
) {
    let event = trigger.event();

    let Ok(dropped_hole) = conveyor_holes.get(event.dropped) else {
        return;
    };

    let Ok(target_hole) = conveyor_holes.get(event.target) else {
        return;
    };

    if dropped_hole == target_hole {
        return;
    }

    let (from, to) = if *dropped_hole == ConveyorHole::Outbound {
        (event.dropped, event.target)
    } else {
        (event.target, event.dropped)
    };

    events.write(QueueConveyorSpawn(from, to));

    trigger.propagate(false);
}
