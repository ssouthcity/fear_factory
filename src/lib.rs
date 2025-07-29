use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod animation;
mod audio;
mod info;
mod input;
mod machine;
mod power;
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
            power::plugin,
            input::plugin,
            machine::plugin,
            info::plugin,
            ui::plugin,
        ));

        app.insert_resource(ClearColor(Color::linear_rgb(0.25, 0.25, 0.0)));

        app.add_systems(Startup, (setup_camera,));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
