use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::{
    gameplay::{
        FactorySystems,
        hud::hotbar::{HotbarActionKind, HotbarSelection, HotbarSelectionChanged},
        recipe::select::SelectRecipe,
        sprite_sort::{YSortSprite, ZIndexSprite},
        structure::{Structure, assets::StructureDef, interactable::Interactable},
        world::{
            deposit::DepositRecipe,
            tilemap::{TILE_OFFSET, TILE_SIZE, map::TerrainClick},
        },
    },
    input::cursor::CursorPosition,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ConstructionPreview>();

    app.register_type::<StructureConstructed>();
    app.add_event::<StructureConstructed>();

    app.add_systems(
        Update,
        (
            (despawn_preview, spawn_preview)
                .chain()
                .run_if(on_event::<HotbarSelectionChanged>),
            move_preview,
        )
            .in_set(FactorySystems::Construction),
    );

    app.add_systems(
        Update,
        construct
            .in_set(FactorySystems::Construction)
            .run_if(on_event::<TerrainClick>),
    );
}

#[derive(Event, Reflect, Debug)]
pub struct StructureConstructed(pub Entity);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct ConstructionPreview;

fn spawn_preview(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    hotbar_selection: HotbarSelection,
    structure_defs: Res<Assets<StructureDef>>,
) {
    let Some(action) = hotbar_selection.action() else {
        return;
    };

    let sprite_location = match action {
        HotbarActionKind::PlaceStructure(handle) => format!(
            "sprites/structures/{}.aseprite",
            structure_defs.get(handle).unwrap().id.to_owned()
        ),
    };

    commands.spawn((
        Name::new("Construction Preview"),
        ConstructionPreview,
        Sprite::from_color(Color::WHITE.with_alpha(0.5), TILE_SIZE),
        AseAnimation {
            aseprite: asset_server.load(sprite_location),
            animation: Animation::tag("work"),
        },
        YSortSprite,
        ZIndexSprite(10),
    ));
}

fn despawn_preview(mut commands: Commands, previews: Query<Entity, With<ConstructionPreview>>) {
    for preview in previews {
        commands.entity(preview).despawn();
    }
}

fn move_preview(
    cursor_position: Res<CursorPosition>,
    mut preview_query: Query<&mut Transform, With<ConstructionPreview>>,
) {
    for mut transform in preview_query.iter_mut() {
        transform.translation = (cursor_position.0 + Vec2::Y * TILE_OFFSET.y).extend(0.0);
    }
}

fn construct(
    mut events: EventReader<TerrainClick>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    hotbar_selection: HotbarSelection,
    structure_definitions: ResMut<Assets<StructureDef>>,
    deposit_recipes: Query<&DepositRecipe>,
    mut construct_events: EventWriter<StructureConstructed>,
) {
    let Some(HotbarActionKind::PlaceStructure(handle)) = hotbar_selection.action() else {
        return;
    };

    let structure = structure_definitions
        .get(handle)
        .expect("Attempted to spawn non-existent structure");

    for event in events.read() {
        let mut entity = commands.spawn((
            Name::new(structure.name.clone()),
            Transform::from_translation(event.position.extend(0.0)),
            Sprite::sized(TILE_SIZE),
            AseAnimation {
                aseprite: asset_server
                    .load(format!("sprites/structures/{}.aseprite", structure.id)),
                animation: Animation::tag("work"),
            },
            YSortSprite,
            ZIndexSprite(10),
            Structure(handle.clone()),
            Interactable,
        ));

        // TODO: Structure specific logic that remains to be ported to manifest
        if matches!(structure.id.as_str(), "miner") {
            if let Ok(deposit_recipe) = deposit_recipes.get(event.entity) {
                entity.trigger(SelectRecipe(deposit_recipe.0.clone()));
            } else {
                entity.despawn();
            };
        }

        construct_events.write(StructureConstructed(entity.id()));
    }
}
