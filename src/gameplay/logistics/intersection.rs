use bevy::prelude::*;

use crate::gameplay::{
    FactorySystems,
    logistics::path::Pathable,
    structure::Structure,
    world::terrain::{Terrain, Worldly},
    y_sort::YSort,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        spawn_intersection
            .in_set(FactorySystems::Build)
            .run_if(on_event::<Pointer<Click>>),
    );
}

fn spawn_intersection(
    mut events: EventReader<Pointer<Click>>,
    terrain: Single<Entity, With<Terrain>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for event in events.read() {
        if event.target != *terrain {
            continue;
        }

        if event.button != PointerButton::Middle {
            continue;
        }

        commands.spawn((
            Name::new("Intersection"),
            Transform::from_translation(event.hit.position.unwrap_or_default()),
            Sprite::from_image(asset_server.load("sprites/logistics/intersection.png")),
            Worldly,
            Pathable::walkable(),
            Pickable::default(),
            YSort::default(),
            Structure,
        ));
    }
}
