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
    app.register_type::<Pathable>();

    app.add_event::<PathsUpdated>();

    app.add_systems(
        Update,
        spawn_path
            .in_set(FactorySystems::Construction)
            .run_if(on_event::<TileClicked>),
    );
}

#[derive(Event, Reflect)]
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
    mut events: EventReader<TileClicked>,
    mut paths_updated_events: EventWriter<PathsUpdated>,
    hotbar_selection: HotbarSelection,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut constructions: ResMut<Constructions>,
) {
    for event in events.read() {
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
                Coord::new(event.0.x, event.0.y),
                YSortSprite,
                ZIndexSprite(9),
                Demolishable,
            ))
            .id();

        constructions.insert(event.0.xy(), entity);

        paths_updated_events.write(PathsUpdated);
    }
}
