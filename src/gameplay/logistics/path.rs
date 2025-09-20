use bevy::{platform::collections::HashSet, prelude::*};

use crate::gameplay::{
    FactorySystems,
    world::{
        demolition::Demolishable,
        terrain::{Terrain, Worldly},
    },
    y_sort::YSort,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Path>();
    app.register_type::<Pathable>();
    app.register_type::<Paths>();
    app.register_type::<PathsUpdated>();

    app.add_event::<PathsUpdated>();

    app.add_observer(add_edges_to_node);
    app.add_observer(remove_edges_from_node);

    app.add_systems(
        Update,
        (
            build_paths
                .in_set(FactorySystems::Build)
                .run_if(on_event::<Pointer<DragDrop>>),
            spawn_intersection
                .in_set(FactorySystems::Build)
                .run_if(on_event::<Pointer<Click>>),
        ),
    );
}

#[derive(Event, Reflect)]
pub struct PathsUpdated;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Path(pub Entity, pub Entity);

impl Path {
    pub fn other(&self, entity: Entity) -> Entity {
        if self.0 == entity { self.1 } else { self.0 }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Paths)]
pub struct Pathable {
    pub walkable: bool,
}

impl Pathable {
    pub fn walkable() -> Self {
        Self { walkable: true }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Paths(pub HashSet<Entity>);

fn add_edges_to_node(
    trigger: Trigger<OnAdd, Path>,
    path_query: Query<&Path>,
    mut edges_query: Query<&mut Paths>,
) {
    let Ok(Path(lhs, rhs)) = path_query.get(trigger.target()) else {
        return;
    };

    if let Ok(mut edges) = edges_query.get_mut(*lhs) {
        edges.0.insert(trigger.target());
    }

    if let Ok(mut edges) = edges_query.get_mut(*rhs) {
        edges.0.insert(trigger.target());
    }
}

fn remove_edges_from_node(
    trigger: Trigger<OnRemove, Path>,
    path_query: Query<&Path>,
    mut edges_query: Query<&mut Paths>,
) {
    let Ok(Path(lhs, rhs)) = path_query.get(trigger.target()) else {
        return;
    };

    if let Ok(mut edges) = edges_query.get_mut(*lhs) {
        edges.0.remove(&trigger.target());
    }

    if let Ok(mut edges) = edges_query.get_mut(*rhs) {
        edges.0.remove(&trigger.target());
    }
}

fn build_paths(
    mut events: EventReader<Pointer<DragDrop>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    pathables: Query<Entity, With<Pathable>>,
    transforms: Query<&Transform>,
    mut paths_updated_events: EventWriter<PathsUpdated>,
) {
    for event in events.read() {
        let target = event.target;
        let dropped = event.dropped;

        if !pathables.contains(target) || !pathables.contains(dropped) {
            continue;
        }

        let Ok(from) = transforms.get(target) else {
            continue;
        };

        let Ok(to) = transforms.get(dropped) else {
            continue;
        };

        let direction = to.translation - from.translation;
        let rotation = Quat::from_rotation_z(direction.xy().to_angle());

        commands.spawn((
            Name::new("Path"),
            Path(target, dropped),
            Worldly,
            Transform::default()
                .with_translation(from.translation.midpoint(to.translation))
                .with_rotation(rotation),
            Sprite {
                image: asset_server.load("sprites/logistics/path.png"),
                custom_size: Some(Vec2::new(
                    to.translation.distance(from.translation) - 64.0,
                    32.0,
                )),
                image_mode: SpriteImageMode::Sliced(TextureSlicer {
                    border: BorderRect {
                        left: 16.0,
                        right: 16.0,
                        top: 0.0,
                        bottom: 0.0,
                    },
                    center_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
                    sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
                    max_corner_scale: 1.0,
                }),
                ..default()
            },
            YSort(0.5),
            Demolishable,
            Pickable::default(),
        ));

        paths_updated_events.write(PathsUpdated);
    }
}

fn spawn_intersection(
    mut events: EventReader<Pointer<Click>>,
    terrain: Single<Entity, With<Terrain>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut paths_updated_events: EventWriter<PathsUpdated>,
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
            Demolishable,
        ));

        paths_updated_events.write(PathsUpdated);
    }
}
