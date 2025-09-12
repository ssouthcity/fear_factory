use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::{
    assets::indexing::IndexMap,
    gameplay::{
        FactorySystems,
        hud::{
            hotbar::{HotbarItemDeselected, HotbarItemSelected},
            inspect::Inspect,
        },
        recipe::select::SelectRecipe,
        structure::{
            Structure,
            assets::StructureDef,
            interactable::{Interact, Interactable},
        },
        world::{deposit::DepositRecipe, terrain::Terrain},
        y_sort::YSort,
    },
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
            aseprite: asset_server.load(format!("sprites/structures/{}.aseprite", trigger.0)),
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
    pub structure_id: String,
    pub position: Vec2,
    pub placed_on: Entity,
}

fn spawn_structures(
    mut events: EventReader<QueueStructureSpawn>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    structure_definitions: Res<Assets<StructureDef>>,
    structure_index: Res<IndexMap<StructureDef>>,
    terrain: Single<Entity, With<Terrain>>,
    deposit_recipes: Query<&DepositRecipe>,
) {
    for event in events.read() {
        let asset_id = structure_index
            .get(&event.structure_id)
            .expect("Attempted to spawn non-indexed structure");

        let structure = structure_definitions
            .get(*asset_id)
            .expect("Attempted to spawn non-existent structure");

        let mut entity = commands.spawn((
            Name::new(structure.name.clone()),
            // position
            Transform::from_translation(event.position.extend(1.0)),
            ChildOf(*terrain),
            // appearance
            Sprite::sized(Vec2::splat(64.0)),
            AseAnimation {
                aseprite: asset_server
                    .load(format!("sprites/structures/{}.aseprite", structure.id)),
                animation: Animation::tag("work"),
            },
            YSort::default(),
            // labels
            Structure,
            Interactable,
        ));

        // TODO: Structure specific logic that remains to be ported to manifest
        match structure.id.as_str() {
            "constructor" | "smelter" => {
                if let Some(recipe) = &structure.default_recipe {
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
