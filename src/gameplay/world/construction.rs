use std::collections::HashMap;

use bevy::{prelude::*, sprite::Anchor};
use bevy_aseprite_ultra::prelude::*;

use crate::{
    gameplay::{
        FactorySystems,
        hud::hotbar::{HotbarActionKind, HotbarSelection, HotbarSelectionChanged},
        sprite_sort::{YSortSprite, ZIndexSprite},
        structure::{
            Structure, assets::StructureDef, harvest::Harvester, interactable::Interactable,
            range::Range,
        },
        world::{
            demolition::Demolished,
            tilemap::{
                TileClicked,
                coord::{Coord, translation_to_coord},
            },
        },
    },
    input::cursor::CursorPosition,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Constructions>();
    app.init_resource::<ValidPlacement>();

    app.add_message::<StructureConstructed>();

    app.add_systems(
        FixedUpdate,
        (
            (despawn_preview, spawn_preview)
                .chain()
                .run_if(on_message::<HotbarSelectionChanged>),
            calculate_valid_placement,
            (move_preview, color_preview),
        )
            .in_set(FactorySystems::Construction),
    );

    app.add_systems(
        FixedUpdate,
        construct
            .in_set(FactorySystems::Construction)
            .run_if(on_message::<TileClicked>),
    );

    app.add_systems(
        FixedUpdate,
        remove_demolished_constructions.after(FactorySystems::Demolish),
    );
}

#[derive(Message, Reflect, Debug)]
pub struct StructureConstructed(pub Entity);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct ConstructionPreview;

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct ValidPlacement(pub bool);

#[derive(Resource, Reflect, Debug, Default, Deref, DerefMut)]
#[reflect(Resource)]
pub struct Constructions(pub HashMap<IVec2, Entity>);

fn spawn_preview(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    hotbar_selection: HotbarSelection,
    structure_defs: Res<Assets<StructureDef>>,
) {
    let Some(action) = hotbar_selection.action() else {
        return;
    };

    let id = commands
        .spawn((
            Name::new("Construction Preview"),
            ConstructionPreview,
            Coord(IVec2::ZERO),
            Sprite {
                color: Color::WHITE.with_alpha(0.5),
                ..default()
            },
            YSortSprite,
            ZIndexSprite(10),
        ))
        .id();

    match action {
        HotbarActionKind::PlaceStructure(handle) => {
            let sprite_path = format!(
                "sprites/structures/{}.aseprite",
                structure_defs.get(handle).unwrap().id.to_owned()
            );

            commands.entity(id).insert((
                Anchor(Vec2::new(0.0, -0.33)),
                AseAnimation {
                    aseprite: asset_server.load(sprite_path),
                    animation: Animation::tag("work"),
                },
            ));
        }
        HotbarActionKind::PlacePath => {
            commands.entity(id).insert(AseSlice {
                aseprite: asset_server.load("sprites/logistics/path_segments.aseprite"),
                name: "C".into(),
            });
        }
    };
}

fn despawn_preview(mut commands: Commands, previews: Query<Entity, With<ConstructionPreview>>) {
    for preview in previews {
        commands.entity(preview).despawn();
    }
}

fn calculate_valid_placement(
    preview: Single<&Coord, With<ConstructionPreview>>,
    constructions: Res<Constructions>,
    mut valid_placement: ResMut<ValidPlacement>,
) {
    let spot_occupied = constructions.contains_key(*preview);
    valid_placement.0 = !spot_occupied;
}

fn move_preview(
    cursor_position: Res<CursorPosition>,
    mut preview_query: Query<&mut Coord, With<ConstructionPreview>>,
) {
    for mut coord in preview_query.iter_mut() {
        *coord = translation_to_coord(&cursor_position.0);
    }
}

fn color_preview(
    valid_placement: Res<ValidPlacement>,
    mut preview_query: Query<&mut Sprite, With<ConstructionPreview>>,
) {
    for mut sprite in preview_query.iter_mut() {
        sprite.color = if !valid_placement.0 {
            Color::hsl(0.0, 1.0, 0.5)
        } else {
            Color::default().with_alpha(0.5)
        };
    }
}

fn construct(
    mut tile_clicks: MessageReader<TileClicked>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    hotbar_selection: HotbarSelection,
    structure_definitions: ResMut<Assets<StructureDef>>,
    mut constructions: ResMut<Constructions>,
    mut structures_constructed: MessageWriter<StructureConstructed>,
) {
    let Some(HotbarActionKind::PlaceStructure(handle)) = hotbar_selection.action() else {
        return;
    };

    let structure = structure_definitions
        .get(handle)
        .expect("Attempted to spawn non-existent structure");

    for tile_click in tile_clicks.read() {
        let entity = commands
            .spawn((
                Name::new(structure.name.clone()),
                Coord(IVec2::new(tile_click.0.x, tile_click.0.y)),
                Anchor(Vec2::new(0.0, -0.33)),
                Sprite::default(),
                AseAnimation {
                    aseprite: asset_server
                        .load(format!("sprites/structures/{}.aseprite", structure.id)),
                    animation: Animation::tag("work"),
                },
                YSortSprite,
                ZIndexSprite(10),
                Structure(handle.clone()),
                Interactable,
            ))
            .id();

        if structure.id == "harvester" {
            commands
                .entity(entity)
                .insert((Harvester, Range::Diamond(4)));
        }

        constructions.insert(tile_click.0.xy(), entity);

        structures_constructed.write(StructureConstructed(entity));
    }
}

fn remove_demolished_constructions(
    mut demolitions: MessageReader<Demolished>,
    mut constructions: ResMut<Constructions>,
) {
    for Demolished { coord, .. } in demolitions.read() {
        constructions.remove(&coord.xy());
    }
}
