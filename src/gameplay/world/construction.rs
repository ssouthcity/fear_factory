use std::collections::HashMap;

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::{
    gameplay::{
        FactorySystems,
        hud::hotbar::{HotbarActionKind, HotbarSelection, HotbarSelectionChanged},
        sprite_sort::{YSortSprite, ZIndexSprite},
        structure::{Structure, assets::StructureDef, interactable::Interactable},
        world::{
            demolition::Demolished,
            tilemap::{
                TILE_OFFSET, TILE_SIZE,
                coord::Coord,
                map::{HoveredTile, TileClicked},
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
        Update,
        (
            (despawn_preview, spawn_preview)
                .chain()
                .run_if(on_message::<HotbarSelectionChanged>),
            calculate_valid_placement.run_if(resource_changed::<HoveredTile>),
            (move_preview, color_preview),
        )
            .in_set(FactorySystems::Construction),
    );

    app.add_systems(
        Update,
        construct
            .in_set(FactorySystems::Construction)
            .run_if(on_message::<TileClicked>),
    );

    app.add_systems(
        Update,
        remove_demolished_constructions.in_set(FactorySystems::PostDemolition),
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
pub struct Constructions(pub HashMap<UVec2, Entity>);

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
            Sprite::from_color(Color::WHITE.with_alpha(0.5), TILE_SIZE),
            YSortSprite,
            ZIndexSprite(100),
        ))
        .id();

    match action {
        HotbarActionKind::PlaceStructure(handle) => {
            let sprite_path = format!(
                "sprites/structures/{}.aseprite",
                structure_defs.get(handle).unwrap().id.to_owned()
            );

            commands.entity(id).insert(AseAnimation {
                aseprite: asset_server.load(sprite_path),
                animation: Animation::tag("work"),
            });
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
    hovered_tile: Res<HoveredTile>,
    constructions: Res<Constructions>,
    tile_query: Query<&TilePos>,
    mut valid_placement: ResMut<ValidPlacement>,
) {
    let Ok(spot_occupied) = tile_query
        .get(hovered_tile.0)
        .map(|tile_pos| constructions.contains_key(&UVec2::from(tile_pos)))
    else {
        return;
    };

    valid_placement.0 = !spot_occupied;
}

fn move_preview(
    cursor_position: Res<CursorPosition>,
    mut preview_query: Query<&mut Transform, With<ConstructionPreview>>,
) {
    for mut transform in preview_query.iter_mut() {
        transform.translation = (cursor_position.0 + Vec2::Y * TILE_OFFSET.y).extend(0.0);
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
            Color::default()
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
                Coord::new(tile_click.0.x, tile_click.0.y),
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
