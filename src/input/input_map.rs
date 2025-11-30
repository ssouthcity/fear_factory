use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

// Basic Interactions
pub const DEFAULT_KEY_INTERACT: KeyCode = KeyCode::KeyE;
pub const DEFAULT_KEY_DISMISS: KeyCode = KeyCode::Escape;
pub const DEFAULT_KEY_OPEN_TOME: KeyCode = KeyCode::KeyT;

// Hotbar
pub const DEFAULT_KEY_HOTBAR_1: KeyCode = KeyCode::Digit1;
pub const DEFAULT_KEY_HOTBAR_2: KeyCode = KeyCode::Digit2;
pub const DEFAULT_KEY_HOTBAR_3: KeyCode = KeyCode::Digit3;
pub const DEFAULT_KEY_HOTBAR_4: KeyCode = KeyCode::Digit4;
pub const DEFAULT_KEY_HOTBAR_5: KeyCode = KeyCode::Digit5;
pub const DEFAULT_KEY_HOTBAR_6: KeyCode = KeyCode::Digit6;
pub const DEFAULT_KEY_HOTBAR_7: KeyCode = KeyCode::Digit7;
pub const DEFAULT_KEY_HOTBAR_8: KeyCode = KeyCode::Digit8;
pub const DEFAULT_KEY_HOTBAR_9: KeyCode = KeyCode::Digit9;

pub const DEFAULT_KEY_DEMOLISH: KeyCode = KeyCode::KeyF;
pub const DEFAULT_KEY_MULTI_SELECT: KeyCode = KeyCode::ShiftLeft;

pub const DEFAULT_KEY_DEBUG_MODE: KeyCode = KeyCode::Backquote;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Reflect, Debug)]
pub enum Action {
    Interact,
    Dismiss,
    OpenTome,

    Hotbar1,
    Hotbar2,
    Hotbar3,
    Hotbar4,
    Hotbar5,
    Hotbar6,
    Hotbar7,
    Hotbar8,
    Hotbar9,

    Demolish,
    MultiSelect,

    DebugMode,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<InputMap>();
    app.init_resource::<InputActions>();

    app.add_systems(PreUpdate, populate_keyboard_actions);
    app.add_systems(PostUpdate, clear_actions);
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct InputMap {
    pub keymap: HashMap<Action, KeyCode>,
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct InputActions {
    pub just_pressed: HashSet<Action>,
    pub pressed: HashSet<Action>,
}

impl Default for InputMap {
    fn default() -> Self {
        Self {
            keymap: HashMap::from([
                // Basic Interactions
                (Action::Interact, DEFAULT_KEY_INTERACT),
                (Action::Dismiss, DEFAULT_KEY_DISMISS),
                (Action::OpenTome, DEFAULT_KEY_OPEN_TOME),
                // Hotbar
                (Action::Hotbar1, DEFAULT_KEY_HOTBAR_1),
                (Action::Hotbar2, DEFAULT_KEY_HOTBAR_2),
                (Action::Hotbar3, DEFAULT_KEY_HOTBAR_3),
                (Action::Hotbar4, DEFAULT_KEY_HOTBAR_4),
                (Action::Hotbar5, DEFAULT_KEY_HOTBAR_5),
                (Action::Hotbar6, DEFAULT_KEY_HOTBAR_6),
                (Action::Hotbar7, DEFAULT_KEY_HOTBAR_7),
                (Action::Hotbar8, DEFAULT_KEY_HOTBAR_8),
                (Action::Hotbar9, DEFAULT_KEY_HOTBAR_9),
                // Demolish
                (Action::Demolish, DEFAULT_KEY_DEMOLISH),
                (Action::MultiSelect, DEFAULT_KEY_MULTI_SELECT),
                // Debug
                (Action::DebugMode, DEFAULT_KEY_DEBUG_MODE),
            ]),
        }
    }
}

fn populate_keyboard_actions(
    input_map: Res<InputMap>,
    mut input_actions: ResMut<InputActions>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (action, key) in input_map.keymap.iter() {
        if keys.just_pressed(*key) {
            input_actions.just_pressed.insert(*action);
        }

        if keys.pressed(*key) {
            input_actions.pressed.insert(*action);
        }
    }
}

fn clear_actions(mut input_actions: ResMut<InputActions>) {
    input_actions.just_pressed.clear();
    input_actions.pressed.clear();
}

pub fn action_just_pressed(action: Action) -> impl SystemCondition<()> {
    IntoSystem::into_system(move |input_actions: Res<InputActions>| {
        input_actions.just_pressed.contains(&action)
    })
}

#[allow(dead_code)]
pub fn action_pressed(action: Action) -> impl SystemCondition<()> {
    IntoSystem::into_system(move |input_actions: Res<InputActions>| {
        input_actions.pressed.contains(&action)
    })
}
