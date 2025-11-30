use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::dev_tools::states::log_transitions;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::input::input_map::{Action, action_just_pressed};
use crate::screens::Screen;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        FpsOverlayPlugin::default(),
        EguiPlugin::default(),
        WorldInspectorPlugin::new().run_if(in_state(DebugMode(true))),
    ));

    app.add_systems(Update, log_transitions::<Screen>);

    app.init_state::<DebugMode>();

    app.add_systems(
        Update,
        toggle_debug_mode.run_if(action_just_pressed(Action::DebugMode)),
    );

    app.add_systems(Update, sync_ui_debug.run_if(state_changed::<DebugMode>));
}

#[derive(States, PartialEq, Eq, Debug, Hash, Reflect, Clone, Default)]
pub struct DebugMode(pub bool);

fn toggle_debug_mode(
    debug_mode: Res<State<DebugMode>>,
    mut next_debug_mode: ResMut<NextState<DebugMode>>,
) {
    next_debug_mode.set(DebugMode(!debug_mode.0));
}

fn sync_ui_debug(mut options: ResMut<UiDebugOptions>, debug_mode: Res<State<DebugMode>>) {
    options.enabled = debug_mode.0;
}
