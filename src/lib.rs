#![allow(clippy::type_complexity)]
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::screens::Screen;

mod assets;
mod audio;
mod camera;
mod dismantle;
mod item;
mod logistics;
mod machine;
mod power;
mod recipe;
mod screens;
mod theme;
mod ui;
mod world;

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

        app.add_plugins(EguiPlugin::default());
        app.add_plugins(WorldInspectorPlugin::new());

        app.add_plugins(AsepriteUltraPlugin);

        app.add_plugins((
            assets::plugin,
            audio::plugin,
            camera::plugin,
            item::plugin,
            dismantle::plugin,
            logistics::plugin,
            machine::plugin,
            power::plugin,
            recipe::plugin,
            world::plugin,
            screens::plugin,
            ui::plugin,
        ));

        app.insert_resource(ClearColor(Color::BLACK));

        app.configure_sets(
            Update,
            (
                FactorySystems::Input,
                FactorySystems::Build,
                FactorySystems::Power,
                FactorySystems::Logistics,
                FactorySystems::Work,
                FactorySystems::Dismantle,
                FactorySystems::UI,
            )
                .chain()
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum FactorySystems {
    Input,
    Build,
    Power,
    Logistics,
    Work,
    Dismantle,
    UI,
}
