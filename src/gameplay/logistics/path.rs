use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::gameplay::{
    FactorySystems,
    hud::hotbar::{HotbarActionKind, HotbarSelection},
    sprite_sort::{YSortSprite, ZIndexSprite},
    world::{
        construction::{Constructions, StructureConstructed},
        demolition::{Demolishable, Demolished},
        tilemap::{
            TILE_OFFSET,
            coord::{Coord, CoordOffset},
            map::TileClicked,
        },
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        spawn_path
            .in_set(FactorySystems::Construction)
            .run_if(on_message::<TileClicked>),
    );

    app.add_systems(
        Update,
        (pick_path_sprite, update_path_segments_on_destroy).in_set(FactorySystems::UI),
    );

    app.add_observer(compute_sprite);
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Pathable {
    pub walkable: bool,
}

impl Pathable {
    pub fn walkable() -> Self {
        Self { walkable: true }
    }
}

fn spawn_path(
    mut tile_clicks: MessageReader<TileClicked>,
    mut path_updates: MessageWriter<StructureConstructed>,
    hotbar_selection: HotbarSelection,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut constructions: ResMut<Constructions>,
) {
    for TileClicked(coord) in tile_clicks.read() {
        let Some(action) = hotbar_selection.action() else {
            continue;
        };

        if !matches!(action, HotbarActionKind::PlacePath) {
            continue;
        }

        let entity = commands
            .spawn((
                Name::new("Path"),
                Pathable::walkable(),
                Sprite::sized(TILE_OFFSET),
                AseSlice {
                    aseprite: asset_server.load("sprites/logistics/path_segments.aseprite"),
                    name: "C".into(),
                },
                Coord::new(coord.x, coord.y),
                CoordOffset(Vec2::new(0.0, -32.0)),
                YSortSprite,
                ZIndexSprite(9),
                Demolishable,
            ))
            .id();

        constructions.insert(coord.xy(), entity);

        path_updates.write(StructureConstructed(entity));
    }
}

fn pick_path_sprite(
    mut structures_constructed: MessageReader<StructureConstructed>,
    mut coord_query: Query<&Coord>,
    constructions: Res<Constructions>,
    mut commands: Commands,
) {
    for StructureConstructed(entity) in structures_constructed.read() {
        let Ok(coord) = coord_query.get_mut(*entity) else {
            continue;
        };

        let coords = [
            coord.0,
            coord.0.saturating_add(UVec2::Y),
            coord.0.saturating_add(UVec2::X),
            coord.0.saturating_sub(UVec2::Y),
            coord.0.saturating_sub(UVec2::X),
        ];

        for coord in coords {
            if let Some(&entity) = constructions.get(&coord) {
                commands.trigger(ComputePathSegmentSprite { entity });
            }
        }
    }
}

fn update_path_segments_on_destroy(
    mut demolitions: MessageReader<Demolished>,
    constructions: Res<Constructions>,
    mut commands: Commands,
) {
    for Demolished { coord, .. } in demolitions.read() {
        let coords = [
            coord.0,
            coord.0.saturating_add(UVec2::Y),
            coord.0.saturating_add(UVec2::X),
            coord.0.saturating_sub(UVec2::Y),
            coord.0.saturating_sub(UVec2::X),
        ];

        for coord in coords {
            if let Some(&entity) = constructions.get(&coord) {
                commands.trigger(ComputePathSegmentSprite { entity });
            }
        }
    }
}

#[derive(EntityEvent, Reflect, Debug)]
pub struct ComputePathSegmentSprite {
    pub entity: Entity,
}

fn compute_sprite(
    compute_path_segment_sprite: On<ComputePathSegmentSprite>,
    mut path_query: Query<(&Coord, &mut AseSlice), With<Pathable>>,
    constructions: Res<Constructions>,
) {
    let Ok((coord, mut aseslice)) = path_query.get_mut(compute_path_segment_sprite.entity) else {
        return;
    };

    let dirs = ['N', 'E', 'S', 'W'];

    let coords = [
        coord.0.saturating_add(UVec2::Y),
        coord.0.saturating_add(UVec2::X),
        coord.0.saturating_sub(UVec2::Y),
        coord.0.saturating_sub(UVec2::X),
    ];

    let name = coords
        .into_iter()
        .map(|cardinal| constructions.get(&cardinal))
        .enumerate()
        .flat_map(|(i, e)| if e.is_some() { Some(dirs[i]) } else { None })
        .collect::<String>();

    let name = if name.is_empty() {
        String::from("C")
    } else if name == "W" && name == "E" {
        String::from("EW")
    } else if name == "N" && name == "S" {
        String::from("NS")
    } else {
        name
    };

    aseslice.name = name;
}
