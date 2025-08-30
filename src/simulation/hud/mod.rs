use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::screens::Screen;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_hud);
}

fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        container(asset_server.load("hud/box.aseprite")),
        children![
            portrait(asset_server.load("hud/portrait.aseprite")),
            header("Michael"),
            content("I've noticed your effort in the mining operations, the village is really warming up to you!"),
            indicator(),
        ],
    ));
}

fn container(aseprite: Handle<Aseprite>) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Auto,
            position_type: PositionType::Absolute,
            bottom: Val::ZERO,
            padding: UiRect::all(Val::Px(32.0)),
            display: Display::Grid,
            grid_template_columns: vec![
                RepeatedGridTrack::auto(1),
                RepeatedGridTrack::flex(1, 1.0),
                RepeatedGridTrack::auto(1),
            ],
            grid_template_rows: vec![
                RepeatedGridTrack::auto(1),
                RepeatedGridTrack::flex(1, 1.0),
                RepeatedGridTrack::auto(1),
            ],
            column_gap: Val::Px(16.0),
            row_gap: Val::Px(16.0),
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
            aseprite,
            name: "box".to_string(),
        },
    )
}

fn header(text: impl Into<String>) -> impl Bundle {
    (
        Text::new(text.into()),
        TextColor(Color::WHITE),
        TextFont {
            font_size: 32.0,
            ..default()
        },
    )
}

fn content(text: impl Into<String>) -> impl Bundle {
    (
        Text::new(text.into()),
        TextColor(Color::WHITE),
        Node {
            grid_row: GridPlacement::auto().set_start(2).set_span(2),
            grid_column: GridPlacement::start(2),
            ..default()
        },
    )
}

fn portrait(aseprite: Handle<Aseprite>) -> impl Bundle {
    (
        Node {
            width: Val::Vh(20.0),
            height: Val::Vh(20.0),
            grid_row: GridPlacement::span(3),
            ..default()
        },
        BackgroundColor(Color::hsl(60.0, 1.0, 0.5)),
        ImageNode::default(),
        AseAnimation {
            aseprite,
            ..default()
        },
    )
}

fn indicator() -> impl Bundle {
    (
        Node {
            grid_column: GridPlacement::start(3),
            grid_row: GridPlacement::start(3),
            ..default()
        },
        Text::new("V"),
    )
}
