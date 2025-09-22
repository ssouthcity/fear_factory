use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::{
    assets::tracking::LoadResource,
    gameplay::world::{MAP_SIZE, WorldSpawnSystems},
    screens::Screen,
};

pub fn plugin(app: &mut App) {
    app.register_type::<TerrainAssets>();
    app.load_resource::<TerrainAssets>();

    app.register_type::<Terrain>();
    app.register_type::<Worldly>();

    app.register_type::<TerrainClick>();
    app.add_event::<TerrainClick>();

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

#[derive(Event, Reflect, Debug)]
pub struct TerrainClick {
    pub entity: Entity,
    pub position: Vec2,
}

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
        .observe(
            |trigger: Trigger<Pointer<Click>>, mut events: EventWriter<TerrainClick>| {
                events.write(TerrainClick {
                    entity: trigger.target,
                    position: trigger.hit.position.unwrap_or_default().xy(),
                });
            },
        );
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
