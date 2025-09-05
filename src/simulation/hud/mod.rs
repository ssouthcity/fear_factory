pub mod item_slot;

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::screens::Screen;

pub fn plugin(app: &mut App) {
    app.add_plugins((item_slot::plugin,));

    app.add_systems(OnEnter(Screen::Gameplay), spawn_hud);
}

fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Hud"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::FlexEnd,
            ..default()
        },
        Pickable::IGNORE,
        children![(
            Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::Grid,
                grid_template_columns: vec![
                    RepeatedGridTrack::vh(1, 20.0),
                    RepeatedGridTrack::fr(1, 1.0),
                    RepeatedGridTrack::vh(1, 20.0),
                ],
                ..default()
            },
            children![
                (
                    portrait(asset_server.load("hud/portrait.aseprite")),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    }
                ),
                (
                    Node {
                        height: Val::Percent(80.0),
                        align_self: AlignSelf::FlexEnd,
                        ..default()
                    },
                    ImageNode {
                        image_mode: NodeImageMode::Sliced(TextureSlicer {
                            border: BorderRect::all(6.0),
                            max_corner_scale: 160.0,
                            ..default()
                        }),
                        ..default()
                    },
                    AseSlice {
                        aseprite: asset_server.load("hud/box.aseprite"),
                        name: "box".to_string(),
                    },
                ),
                (
                    portrait(asset_server.load("hud/portrait.aseprite")),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                ),
            ],
        )],
    ));
}

fn portrait(aseprite: Handle<Aseprite>) -> impl Bundle {
    (
        ImageNode::default(),
        AseAnimation {
            aseprite,
            ..default()
        },
    )
}
