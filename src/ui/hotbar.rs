use bevy::{ecs::spawn::SpawnIter, prelude::*};
use bevy_aseprite_ultra::prelude::*;

use crate::{
    FactorySystems,
    world::{Buildable, QueueSpawnBuilding},
};

pub fn plugin(app: &mut App) {
    app.register_type::<HotbarItemSelected>();
    app.register_type::<HotbarItemDeselected>();

    app.register_type::<HotbarSelection>();
    app.register_type::<HotbarAction>();
    app.register_type::<HotbarShortcut>();

    app.init_resource::<HotbarSelection>();

    app.add_systems(Startup, spawn_hotbar);

    app.add_observer(on_hotbar_slot_click);
    app.add_observer(on_slot_selected);

    app.add_systems(
        Update,
        (
            check_for_hotbar_shortcuts.in_set(FactorySystems::Input),
            highlight_selected_slot.in_set(FactorySystems::UI),
            deselect_slot.run_if(on_event::<QueueSpawnBuilding>),
        ),
    );
}

#[derive(Event, Reflect)]
pub struct SelectHotbarSlot(pub Buildable);

#[derive(Event, Reflect)]
pub struct HotbarItemSelected(pub Buildable);

#[derive(Event, Reflect)]
pub struct HotbarItemDeselected;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct HotbarSelection(pub Option<Buildable>);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct HotbarAction(Buildable);

#[derive(Component, Reflect)]
#[reflect(Component)]
struct HotbarShortcut(KeyCode);

fn spawn_hotbar(mut commands: Commands, asset_server: Res<AssetServer>) {
    let aseprite = asset_server.load::<Aseprite>("build-icons.aseprite");

    commands.spawn((
        Name::new("Build Hotbar"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(8.0),
            width: Val::Percent(100.0),
            height: Val::Auto,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.0),
            row_gap: Val::Px(8.0),
            ..default()
        },
        Pickable::IGNORE,
        Children::spawn(SpawnIter(
            [
                (KeyCode::Digit1, Buildable::Windmill, "Windmill"),
                (KeyCode::Digit2, Buildable::PowerPole, "Power Pole"),
                (KeyCode::Digit3, Buildable::Miner, "Miner"),
                (KeyCode::Digit4, Buildable::CoalGenerator, "Coal Generator"),
                (KeyCode::Digit5, Buildable::Constructor, "Constructor"),
            ]
            .iter()
            .map(move |(shortcut, action, slice_name)| {
                (
                    Name::new(format!("Hotbar Slot {:?}", action.clone())),
                    Node {
                        width: Val::Px(64.0),
                        height: Val::Px(64.0),
                        border: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    Pickable::default(),
                    BorderColor(Color::WHITE),
                    HotbarShortcut(*shortcut),
                    HotbarAction(*action),
                    children![(
                        Name::new("Icon"),
                        ImageNode::default(),
                        Pickable::IGNORE,
                        AseSlice {
                            aseprite: aseprite.clone(),
                            name: slice_name.to_string(),
                        },
                    )],
                )
            }),
        )),
    ));
}

fn on_hotbar_slot_click(
    trigger: Trigger<Pointer<Click>>,
    hotbar_actions: Query<&HotbarAction>,
    mut commands: Commands,
) {
    let Ok(action) = hotbar_actions.get(trigger.target) else {
        return;
    };

    commands.trigger(SelectHotbarSlot(action.0));
}

fn check_for_hotbar_shortcuts(
    keys: Res<ButtonInput<KeyCode>>,
    hotbar_slots: Query<(&HotbarAction, &HotbarShortcut)>,
    mut commands: Commands,
) {
    for (action, shortcut) in hotbar_slots {
        if !keys.just_pressed(shortcut.0) {
            continue;
        }

        commands.trigger(SelectHotbarSlot(action.0));
    }
}

fn on_slot_selected(
    trigger: Trigger<SelectHotbarSlot>,
    mut hotbar_selection: ResMut<HotbarSelection>,
    mut commands: Commands,
) {
    let event = trigger.event();

    if hotbar_selection.0 == Some(event.0) {
        hotbar_selection.0 = None;
        commands.trigger(HotbarItemDeselected);
    } else {
        hotbar_selection.0 = Some(event.0);
        commands.trigger(HotbarItemSelected(event.0));
    }
}

fn highlight_selected_slot(
    mut commands: Commands,
    hotbar_slots: Query<(Entity, &HotbarAction)>,
    selection: Res<HotbarSelection>,
) {
    for (entity, slot) in hotbar_slots {
        if selection.0.is_some_and(|b| b == slot.0) {
            commands
                .entity(entity)
                .insert(BackgroundColor(Color::WHITE));
        } else {
            commands.entity(entity).remove::<BackgroundColor>();
        }
    }
}

fn deselect_slot(mut hotbar_selection: ResMut<HotbarSelection>, mut commands: Commands) {
    hotbar_selection.0 = None;
    commands.trigger(HotbarItemDeselected);
}
