use bevy::prelude::*;

use crate::gameplay::{
    FactorySystems,
    hud::hotbar::{HotbarActionKind, HotbarSelection},
    sprite_sort::{YSortSprite, ZIndexSprite},
    world::{
        construction::Constructions,
        demolition::Demolishable,
        tilemap::{coord::Coord, map::TileClicked},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<PathsUpdated>();

    app.add_systems(
        Update,
        spawn_path
            .in_set(FactorySystems::Construction)
            .run_if(on_message::<TileClicked>),
    );
}

#[derive(Message, Reflect)]
pub struct PathsUpdated;

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
    mut path_updates: MessageWriter<PathsUpdated>,
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
                Sprite::from_image(asset_server.load("sprites/logistics/path.png")),
                Coord::new(coord.x, coord.y),
                YSortSprite,
                ZIndexSprite(9),
                Demolishable,
            ))
            .id();

        constructions.insert(coord.xy(), entity);

        path_updates.write(PathsUpdated);
    }
}
