use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod assets;
mod camera;
#[cfg(feature = "dev")]
mod dev_tools;
mod gameplay;
mod input;
mod screens;
mod widgets;

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
        app.add_plugins(TilemapPlugin);

        app.add_plugins((
            assets::plugin,
            camera::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            gameplay::plugin,
            input::plugin,
            screens::plugin,
        ));

        app.insert_resource(ClearColor(Color::BLACK));
    }
}
