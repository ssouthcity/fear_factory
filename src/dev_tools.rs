use bevy::dev_tools::states::log_transitions;
use bevy::input::common_conditions::{input_just_pressed, input_toggle_active};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::screens::Screen;

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

pub fn plugin(app: &mut App) {
    app.add_plugins(EguiPlugin::default());
    app.add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, TOGGLE_KEY)));

    app.add_systems(Update, log_transitions::<Screen>);

    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
