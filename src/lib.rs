#![allow(clippy::type_complexity)]
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use bevy_aseprite_ultra::AsepriteUltraPlugin;

mod assets;
mod audio;
mod camera;
#[cfg(feature = "dev")]
mod dev_tools;
mod screens;
mod simulation;
mod ui;

pub struct FactoryGamePlugin;

impl Plugin for FactoryGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Factory".to_string(),
                        present_mode: PresentMode::AutoNoVsync,
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );

        app.add_plugins(AsepriteUltraPlugin);

        app.add_plugins((
            assets::plugin,
            audio::plugin,
            camera::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            simulation::plugin,
            screens::plugin,
            ui::plugin,
        ));

        app.insert_resource(ClearColor(Color::BLACK));
    }
}
