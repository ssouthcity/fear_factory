use bevy::prelude::*;

use crate::BuildingType;

pub fn plugin(app: &mut App) {
    app.register_type::<HotbarSelection>();
    app.register_type::<HotbarSlot>();

    app.init_resource::<HotbarSelection>();

    app.add_systems(Startup, spawn_hotbar);

    app.add_systems(
        Update,
        (handle_hotbar_input, highlight_selected_slot).chain(),
    );
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct HotbarSelection(pub BuildingType);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct HotbarSlot(BuildingType);

fn spawn_hotbar(mut commands: Commands) {
    commands.spawn((
        Name::new("Taskbar Container"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::ZERO,
            width: Val::Percent(100.0),
            height: Val::Px(128.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.0),
            row_gap: Val::Px(8.0),
            ..default()
        },
        children![
            hotbar_slot(BuildingType::Miner, Color::linear_rgb(0.5, 0.0, 0.0)),
            hotbar_slot(
                BuildingType::CoalGenerator,
                Color::linear_rgb(0.0, 0.0, 0.0)
            ),
            hotbar_slot(BuildingType::PowerPole, Color::linear_rgb(0.0, 0.0, 0.5)),
        ],
    ));
}

fn hotbar_slot(slot: BuildingType, color: Color) -> impl Bundle {
    (
        Name::new(format!("Hotbar Slot")),
        HotbarSlot(slot),
        Node {
            width: Val::Px(64.0),
            height: Val::Px(64.0),
            ..default()
        },
        BackgroundColor(color),
        BorderColor(Color::linear_rgb(0.8, 0.8, 0.0)),
    )
}

fn handle_hotbar_input(keys: Res<ButtonInput<KeyCode>>, mut selected: ResMut<HotbarSelection>) {
    let keycodes = [
        (KeyCode::Digit1, BuildingType::Miner),
        (KeyCode::Digit2, BuildingType::CoalGenerator),
        (KeyCode::Digit3, BuildingType::PowerPole),
    ];

    for (key, building) in keycodes {
        if keys.just_pressed(key) {
            selected.0 = building;
        }
    }
}

fn highlight_selected_slot(
    hotbar_slots: Query<(&HotbarSlot, &mut Node)>,
    selection: Res<HotbarSelection>,
) {
    for (slot, mut node) in hotbar_slots {
        if slot.0 == selection.0 {
            node.border = UiRect::all(Val::Px(4.0));
        } else {
            node.border = UiRect::DEFAULT;
        }
    }
}
