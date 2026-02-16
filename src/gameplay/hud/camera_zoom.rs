use bevy::{prelude::*, ui_widgets::observe};

use crate::{assets::tracking::LoadResource, screens::Screen};

pub fn plugin(app: &mut App) {
    app.load_resource::<CameraZoomAssets>();

    app.add_systems(OnEnter(Screen::Gameplay), spawn_camera_zoom);
}

#[derive(Asset, Resource, Reflect, Clone)]
#[reflect(Resource)]
struct CameraZoomAssets {
    frog_icon: Handle<Image>,
    bird_icon: Handle<Image>,
    space_icon: Handle<Image>,
}

impl FromWorld for CameraZoomAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            frog_icon: asset_server.load("icons/frog.png"),
            bird_icon: asset_server.load("icons/bird.png"),
            space_icon: asset_server.load("icons/space.png"),
        }
    }
}

fn spawn_camera_zoom(mut commands: Commands, camera_zoom_assets: Res<CameraZoomAssets>) {
    commands.spawn((
        Name::new("Camera Zoom Root"),
        Node {
            width: percent(100.0),
            height: percent(100.0),
            padding: px(8.0).all(),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            row_gap: px(8.0),
            ..default()
        },
        children![
            icon(camera_zoom_assets.frog_icon.clone(), 1.0),
            icon(camera_zoom_assets.bird_icon.clone(), 2.0),
            icon(camera_zoom_assets.space_icon.clone(), 4.0),
        ],
    ));
}

fn icon(icon: Handle<Image>, zoom_level: f32) -> impl Bundle {
    (
        Node {
            width: px(64.0),
            height: px(64.0),
            ..default()
        },
        ImageNode::new(icon),
        observe(
            move |_click: On<Pointer<Click>>,
                  mut projection: Single<&mut Projection, With<Camera>>| {
                let Projection::Orthographic(ref mut ortho) = **projection else {
                    return;
                };

                ortho.scale = zoom_level;
            },
        ),
    )
}
