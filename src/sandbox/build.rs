use std::time::Duration;

use bevy::prelude::*;

use crate::{
    FactorySystems,
    assets::manifest::ManifestParam,
    item::{Item, Stack},
    logistics::ResourceOutput,
    machine::work::Frequency,
    prefabs,
    sandbox::{DepositItem, Sandbox},
    ui::{HotbarItemDeselected, HotbarItemSelected, Inspect, Interact, YSort},
};

pub fn plugin(app: &mut App) {
    app.register_type::<Preview>();
    app.register_type::<Building>();
    app.register_type::<Buildable>();
    app.register_type::<QueueSpawnBuilding>();

    app.add_event::<QueueSpawnBuilding>();

    app.add_observer(on_hotbar_selection)
        .add_observer(on_hotbar_deselection);

    app.add_systems(
        Update,
        spawn_buildings
            .run_if(on_event::<QueueSpawnBuilding>)
            .in_set(FactorySystems::Build),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Preview;

fn on_hotbar_selection(
    trigger: Trigger<HotbarItemSelected>,
    mut commands: Commands,
    sandbox: Single<Entity, With<Sandbox>>,
    existing_preview: Option<Single<Entity, With<Preview>>>,
) {
    if let Some(existing) = existing_preview {
        commands.entity(*existing).despawn();
    }

    let common = (Preview::default(), ChildOf(*sandbox), YSort::default());

    match trigger.0 {
        Buildable::Windmill => {
            commands.spawn((prefabs::windmill_preview(), common));
        }
        Buildable::PowerPole => {
            commands.spawn((prefabs::power_pole_preview(), common));
        }
        Buildable::Miner => {
            commands.spawn((prefabs::miner_preview(), common));
        }
        Buildable::CoalGenerator => {
            commands.spawn((prefabs::coal_generator_preview(), common));
        }
        Buildable::Constructor => {
            commands.spawn((prefabs::constructor_preview(), common));
        }
    };
}

fn on_hotbar_deselection(
    _trigger: Trigger<HotbarItemDeselected>,
    preview: Single<Entity, With<Preview>>,
    mut commands: Commands,
) {
    commands.entity(*preview).despawn();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(YSort::default())]
pub struct Building(Buildable);

#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Reflect, Debug, Clone, Copy)]
pub enum Buildable {
    #[default]
    Windmill,
    PowerPole,
    Miner,
    CoalGenerator,
    Constructor,
}

#[derive(Event, Reflect)]
pub struct QueueSpawnBuilding {
    pub buildable: Buildable,
    pub position: Vec2,
    pub placed_on: Entity,
}

fn spawn_buildings(
    mut events: EventReader<QueueSpawnBuilding>,
    mut commands: Commands,
    world: Single<Entity, With<Sandbox>>,
    deposits: Query<&DepositItem>,
    item_manifest: ManifestParam<Item>,
) {
    for event in events.read() {
        let common = (
            Transform::from_translation(event.position.extend(1.0)),
            ChildOf(*world),
            Building(event.buildable),
        );

        match event.buildable {
            Buildable::Windmill => {
                commands.spawn((prefabs::windmill(), common));
            }
            Buildable::PowerPole => {
                commands.spawn((prefabs::power_pole(), common));
            }
            Buildable::Miner => {
                let Ok(deposit) = deposits.get(event.placed_on) else {
                    return;
                };

                let Some(items) = item_manifest.read() else {
                    return;
                };

                let Some(item_definition) = items.get(&deposit.0) else {
                    error!("Deposit refers to non-existent item {:?}", deposit.0);
                    return;
                };

                commands.spawn((
                    prefabs::miner(),
                    common,
                    Frequency(Duration::from_secs_f32(40.0 / 60.0)),
                    ResourceOutput(vec![Stack::from(&item_definition).with_quantity(1)]),
                ));
            }
            Buildable::CoalGenerator => {
                commands.spawn((prefabs::coal_generator(), common));
            }
            Buildable::Constructor => {
                commands.spawn((prefabs::constructor(), common)).observe(
                    |trigger: Trigger<Interact>, mut commands: Commands| {
                        commands.trigger_targets(Inspect, trigger.target());
                    },
                );
            }
        };
    }
}
