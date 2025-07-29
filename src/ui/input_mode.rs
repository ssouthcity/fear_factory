use bevy::{ecs::spawn::SpawnIter, prelude::*};
use bevy_aseprite_ultra::prelude::*;

use crate::input::InputMode;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_input_menu);

    app.add_systems(
        Update,
        highlight_selected_state.run_if(state_changed::<InputMode>),
    );

    app.add_observer(on_input_menu_click);
}

fn spawn_input_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let aseprite = asset_server.load::<Aseprite>("input-mode-icons.aseprite");

    commands.spawn((
        Name::new("Input Mode Hotbar"),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            column_gap: Val::Px(8.0),
            row_gap: Val::Px(8.0),
            position_type: PositionType::Absolute,
            right: Val::Px(8.0),
            width: Val::Auto,
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Children::spawn(SpawnIter(
            [InputMode::View, InputMode::Build, InputMode::PowerLine]
                .iter()
                .map(move |mode| {
                    (
                        Name::new(format!("Hotbar Slot {:?}", mode)),
                        Node {
                            width: Val::Px(64.0),
                            height: Val::Px(64.0),
                            border: UiRect::all(Val::Px(4.0)),
                            ..default()
                        },
                        BorderColor(Color::WHITE),
                        PickInputMode(mode.clone()),
                        children![(
                            Name::new("Icon"),
                            ImageNode::default(),
                            AseSlice {
                                aseprite: aseprite.clone(),
                                name: (match mode {
                                    InputMode::View => "View",
                                    InputMode::Build => "Build",
                                    InputMode::PowerLine => "Power Line",
                                })
                                .to_string(),
                            },
                        )],
                    )
                }),
        )),
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(Pickable)]
struct PickInputMode(InputMode);

fn on_input_menu_click(
    trigger: Trigger<Pointer<Click>>,
    pick_input_modes: Query<&PickInputMode>,
    mut next_input_state: ResMut<NextState<InputMode>>,
) {
    if let Ok(mode) = pick_input_modes.get(trigger.target()) {
        next_input_state.set(mode.0);
    }
}

fn highlight_selected_state(
    mut commands: Commands,
    input_mode_slots: Query<(Entity, &PickInputMode)>,
    state: Res<State<InputMode>>,
) {
    for (slot, mode) in input_mode_slots {
        if *state.get() == mode.0 {
            commands.entity(slot).insert(BackgroundColor(Color::WHITE));
        } else {
            commands.entity(slot).remove::<BackgroundColor>();
        }
    }
}
