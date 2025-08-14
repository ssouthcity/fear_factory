use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::{
    sandbox::{QueueSpawnBuilding, SANDBOX_MAP_SIZE, SandboxSpawnSystems, build::Preview},
    screens::Screen,
    ui::HotbarSelection,
};

pub fn plugin(app: &mut App) {
    app.register_type::<SandboxAssets>();
    app.register_type::<Sandbox>();

    app.init_resource::<SandboxAssets>();

    app.add_systems(Startup, load_sandbox_assets);

    app.add_systems(
        OnEnter(Screen::Gameplay),
        spawn_sandbox
            .after(load_sandbox_assets)
            .in_set(SandboxSpawnSystems::SpawnSandbox),
    );
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct SandboxAssets {
    aseprite: Handle<Aseprite>,
}

fn load_sandbox_assets(mut sandbox_assets: ResMut<SandboxAssets>, asset_server: Res<AssetServer>) {
    sandbox_assets.aseprite = asset_server.load("terrain.aseprite");
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Sandbox;

fn spawn_sandbox(mut commands: Commands, sandbox_assets: Res<SandboxAssets>) {
    commands
        .spawn((
            Name::new("Sandbox"),
            Sandbox::default(),
            Sprite {
                custom_size: Some(Vec2::splat(SANDBOX_MAP_SIZE)),
                image_mode: SpriteImageMode::Tiled {
                    tile_x: true,
                    tile_y: true,
                    stretch_value: 4.0,
                },
                ..default()
            },
            AseSlice {
                aseprite: sandbox_assets.aseprite.clone(),
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
    mut events: EventWriter<QueueSpawnBuilding>,
    selected_buildable: Res<HotbarSelection>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }

    let Some(cursor_position) = trigger.event().hit.position else {
        return;
    };

    let Some(buildable) = selected_buildable.0 else {
        return;
    };

    events.write(QueueSpawnBuilding {
        buildable: buildable,
        position: cursor_position.truncate(),
        placed_on: trigger.target,
    });
}
