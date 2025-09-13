use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::{
    assets::tracking::LoadResource,
    gameplay::{
        hud::hotbar::HotbarSelection,
        structure::build::{Preview, QueueStructureSpawn},
        world::{MAP_SIZE, WorldSpawnSystems},
    },
    screens::Screen,
};

pub fn plugin(app: &mut App) {
    app.register_type::<TerrainAssets>();
    app.load_resource::<TerrainAssets>();

    app.register_type::<Terrain>();
    app.register_type::<Worldly>();

    app.add_systems(
        OnEnter(Screen::Gameplay),
        spawn_terrain.in_set(WorldSpawnSystems::SpawnTerrain),
    );

    app.add_systems(PostUpdate, add_to_world);
}

#[derive(Asset, Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct TerrainAssets {
    pub aseprite: Handle<Aseprite>,
}

impl FromWorld for TerrainAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            aseprite: assets.load("terrain.aseprite"),
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Terrain;

fn spawn_terrain(mut commands: Commands, world_assets: Res<TerrainAssets>) {
    commands
        .spawn((
            Name::new("World"),
            Terrain,
            Sprite {
                custom_size: Some(Vec2::splat(MAP_SIZE)),
                image_mode: SpriteImageMode::Tiled {
                    tile_x: true,
                    tile_y: true,
                    stretch_value: 4.0,
                },
                ..default()
            },
            AseSlice {
                aseprite: world_assets.aseprite.clone(),
                name: "grass".to_string(),
            },
            Pickable::default(),
        ))
        .observe(queue_spawn_building)
        .observe(move_preview);
}

fn move_preview(
    trigger: Trigger<Pointer<Move>>,
    mut preview: Single<&mut Transform, With<Preview>>,
) {
    preview.translation = trigger.hit.position.unwrap();
}

fn queue_spawn_building(
    trigger: Trigger<Pointer<Click>>,
    mut events: EventWriter<QueueStructureSpawn>,
    selected_buildable: Res<HotbarSelection>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }

    let Some(cursor_position) = trigger.event().hit.position else {
        return;
    };

    let Some(ref structure_id) = selected_buildable.0 else {
        return;
    };

    events.write(QueueStructureSpawn {
        structure_id: structure_id.clone(),
        position: cursor_position.truncate(),
        placed_on: trigger.target,
    });
}

/// Denotes that entity should be spawned in the world
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Worldly;

fn add_to_world(
    query: Query<Entity, Added<Worldly>>,
    world: Single<Entity, With<Terrain>>,
    mut commands: Commands,
) {
    for entity in query {
        commands.entity(entity).insert(ChildOf(*world));
    }
}
