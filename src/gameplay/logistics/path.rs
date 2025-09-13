use bevy::{platform::collections::HashSet, prelude::*};

use crate::gameplay::{FactorySystems, world::terrain::Worldly, y_sort::YSort};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Path>();
    app.register_type::<Pathable>();

    app.add_systems(
        Update,
        (build_paths,)
            .in_set(FactorySystems::Build)
            .run_if(on_event::<Pointer<DragDrop>>),
    );

    app.register_type::<PathGraph>()
        .init_resource::<PathGraph>()
        .add_systems(
            Update,
            (add_nodes_to_graph, add_edges_to_graph).in_set(FactorySystems::Logistics),
        );
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct PathGraph {
    nodes: HashSet<Entity>,
    edges: HashSet<(Entity, Entity)>,
}

fn add_nodes_to_graph(query: Query<Entity, Added<Pathable>>, mut graph: ResMut<PathGraph>) {
    for entity in query {
        graph.nodes.insert(entity);
    }
}

fn add_edges_to_graph(query: Query<&Path, Added<Path>>, mut graph: ResMut<PathGraph>) {
    for path in query {
        graph.edges.insert((path.0, path.1));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Path(Entity, Entity);

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

fn build_paths(
    mut events: EventReader<Pointer<DragDrop>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    pathables: Query<Entity, With<Pathable>>,
    transforms: Query<&Transform>,
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
        ));
    }
}
