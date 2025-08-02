use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod animation;
mod audio;
mod build;
mod camera;
mod dismantle;
mod info;
mod logistics;
mod machine;
mod power;
mod sandbox;
mod ui;

pub struct FactoryGamePlugin;

impl Plugin for FactoryGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));

        app.add_plugins(EguiPlugin::default());
        app.add_plugins(WorldInspectorPlugin::new());

        app.add_plugins((
            animation::plugin,
            audio::plugin,
            build::plugin,
            camera::plugin,
            info::plugin,
            dismantle::plugin,
            logistics::plugin,
            machine::plugin,
            power::plugin,
            ui::plugin,
            sandbox::plugin,
        ));

        app.insert_resource(ClearColor(Color::BLACK));

        app.configure_sets(
            Update,
            (
                FactorySystems::Build,
                FactorySystems::GarbageClean,
                FactorySystems::Power,
                FactorySystems::Logistics,
                FactorySystems::Work,
                FactorySystems::UI,
            )
                .chain(),
        );
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum FactorySystems {
    Build,
    GarbageClean,
    Power,
    Logistics,
    Work,
    UI,
}
