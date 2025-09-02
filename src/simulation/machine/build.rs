use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::{
    assets::manifest::{Id, Manifest},
    simulation::{
        FactorySystems,
        logistics::ConveyorHoleOf,
        machine::{
            Machine, Structure,
            assets::{StructureAssets, StructureTemplate},
            power::Powered,
        },
        power::{PowerConsumer, PowerProducer, socket::PowerSockets},
        recipe::SelectRecipe,
        world::{DepositRecipe, Terrain},
    },
    ui::{HotbarItemDeselected, HotbarItemSelected, Inspect, Interact, Interactable, YSort},
};

pub fn plugin(app: &mut App) {
    app.register_type::<Preview>()
        .add_observer(on_hotbar_selection)
        .add_observer(on_hotbar_deselection);

    app.register_type::<QueueStructureSpawn>()
        .add_event::<QueueStructureSpawn>()
        .add_systems(
            Update,
            spawn_structures
                .run_if(on_event::<QueueStructureSpawn>)
                .in_set(FactorySystems::Build),
        );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Preview;

fn on_hotbar_selection(
    trigger: Trigger<HotbarItemSelected>,
    mut commands: Commands,
    terrain: Single<Entity, With<Terrain>>,
    existing_preview: Option<Single<Entity, With<Preview>>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(existing) = existing_preview {
        commands.entity(*existing).despawn();
    }

    commands.spawn((
        Preview,
        ChildOf(*terrain),
        Sprite::from_color(Color::WHITE.with_alpha(0.5), Vec2::splat(64.0)),
        AseAnimation {
            aseprite: asset_server.load(format!("structures/{}.aseprite", trigger.0.value)),
            animation: Animation::tag("work"),
        },
        YSort::default(),
    ));
}

fn on_hotbar_deselection(
    _trigger: Trigger<HotbarItemDeselected>,
    preview: Single<Entity, With<Preview>>,
    mut commands: Commands,
) {
    commands.entity(*preview).despawn();
}

#[derive(Event, Reflect)]
pub struct QueueStructureSpawn {
    pub structure_id: Id<StructureTemplate>,
    pub position: Vec2,
    pub placed_on: Entity,
}

fn spawn_structures(
    mut events: EventReader<QueueStructureSpawn>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    structure_manifests: Res<Assets<Manifest<StructureTemplate>>>,
    structure_assets: Res<StructureAssets>,
    terrain: Single<Entity, With<Terrain>>,
    deposit_recipes: Query<&DepositRecipe>,
) {
    let structures = structure_manifests
        .get(&structure_assets.manifest)
        .expect("Structure manifest is not loaded");

    for event in events.read() {
        let structure = structures
            .get(&event.structure_id)
            .expect("Attempted to spawn non-existent structure");

        let mut entity = commands.spawn((
            Name::new(structure.name.clone()),
            // position
            Transform::from_translation(event.position.extend(1.0)),
            ChildOf(*terrain),
            // appearance
            Sprite::sized(Vec2::splat(64.0)),
            AseAnimation {
                aseprite: asset_server.load(format!("structures/{}.aseprite", structure.id.value)),
                animation: Animation::tag("work"),
            },
            YSort::default(),
            // power
            PowerSockets::single(),
            // labels
            Structure(structure.id.clone()),
            Machine,
            Powered,
            Interactable,
        ));

        if let Some(power) = &structure.power {
            if let Some(sockets) = power.sockets {
                entity.insert(PowerSockets::multiple(sockets));
            }

            if let Some(production) = power.production {
                entity.insert(PowerProducer(production));
            }

            if let Some(consumption) = power.consumption {
                entity.insert(PowerConsumer(consumption));
            }
        }

        for hole in structure.conveyor_holes.iter() {
            entity.with_related::<ConveyorHoleOf>((
                Name::new("Conveyor Hole"),
                Transform::from_translation(hole.translation),
                hole.direction.clone(),
            ));
        }

        // TODO: Structure specific logic that remains to be ported to manifest
        match structure.id.value.as_str() {
            "constructor" | "smelter" => {
                if let Some(ref recipe) = structure
                    .recipe
                    .as_ref()
                    .and_then(|r| r.default_recipe.to_owned())
                {
                    entity.trigger(SelectRecipe(recipe.to_owned()));
                }

                entity.observe(|trigger: Trigger<Interact>, mut commands: Commands| {
                    commands.trigger_targets(Inspect, trigger.target());
                });
            }
            "miner" => {
                if let Ok(deposit_recipe) = deposit_recipes.get(event.placed_on) {
                    entity.trigger(SelectRecipe(deposit_recipe.0.clone()));
                } else {
                    entity.despawn();
                };
            }
            _ => {}
        };
    }
}
