use bevy::{
    input_focus::{InputDispatchPlugin, tab_navigation::TabNavigationPlugin},
    prelude::*,
    ui_widgets::UiWidgetsPlugins,
};
use bevy_aseprite_ultra::prelude::*;

use crate::screens::Screen;

pub mod hotbar;
pub mod inspect;
pub mod tome;

pub fn plugin(app: &mut App) {
    app.add_plugins((UiWidgetsPlugins, InputDispatchPlugin, TabNavigationPlugin));

    app.add_plugins((hotbar::plugin, inspect::plugin, tome::plugin));

    app.add_systems(OnEnter(Screen::Gameplay), (setup_portrait, setup_relic));
}

fn setup_portrait(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: vw(15.0),
            height: auto(),
            position_type: PositionType::Absolute,
            bottom: percent(0.0),
            left: percent(0.0),
            ..default()
        },
        ImageNode::default(),
        AseAnimation {
            aseprite: asset_server.load("sprites/hud/player_portrait.aseprite"),
            animation: Animation::tag("thinking"),
        },
    ));
}

fn setup_relic(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: vw(15.0),
            height: auto(),
            position_type: PositionType::Absolute,
            bottom: percent(0.0),
            right: percent(0.0),
            ..default()
        },
        ImageNode::default(),
        AseAnimation {
            aseprite: asset_server.load("sprites/hud/hand.aseprite"),
            animation: Animation::tag("fidget"),
        },
    ));
}
