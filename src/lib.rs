use bevy::prelude::*;

use crate::hotbar::HotbarSelection;

mod hotbar;

pub struct FactoryGamePlugin;

impl Plugin for FactoryGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);

        app.add_plugins(hotbar::plugin);

        app.insert_resource(ClearColor(Color::linear_rgb(0.1, 0.1, 0.1)));

        app.add_systems(Startup, (setup_camera, setup_world));

        app.add_observer(drag_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_world(mut commands: Commands) {
    commands
        .spawn((
            Sprite::from_color(Color::linear_rgb(0.25, 0.25, 0.0), Vec2::splat(1028.0)),
            Pickable::default(),
        ))
        .observe(spawn_building);
}

fn drag_camera(
    trigger: Trigger<Pointer<Drag>>,
    mut camera_position: Single<&mut Transform, With<Camera>>,
) {
    let event = trigger.event();

    if event.button != PointerButton::Secondary {
        return;
    }

    camera_position.translation += event.delta.extend(0.0) * Vec3::new(-1.0, 1.0, 1.0) * 0.2;
}

fn spawn_building(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    selected_buildable: Res<HotbarSelection>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }

    let mouse_position = trigger.event().hit.position.unwrap_or_default();

    let mut entity = commands.spawn(Transform::from_translation(mouse_position));

    match selected_buildable.0 {
        BuildingType::Miner => entity.insert(Miner),
        BuildingType::PowerPole => entity.insert(PowerPole),
    };
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Reflect)]
pub enum BuildingType {
    #[default]
    Miner,
    PowerPole,
}

#[derive(Component)]
#[require(Sprite::from_color(Color::linear_rgb(0.5, 0.0, 0.0), Vec2::splat(64.0)))]
struct Miner;

#[derive(Component)]
#[require(Sprite::from_color(Color::linear_rgb(0.0, 0.0, 0.5), Vec2::splat(64.0)))]
struct PowerPole;
