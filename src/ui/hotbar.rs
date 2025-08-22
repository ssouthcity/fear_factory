use bevy::{ecs::spawn::SpawnIter, prelude::*};
use bevy_aseprite_ultra::prelude::*;

use crate::{
    FactorySystems,
    assets::manifest::Id,
    machine::{QueueStructureSpawn, StructureTemplate},
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
            update_icon.in_set(FactorySystems::UI),
            deselect_slot.run_if(on_event::<QueueStructureSpawn>),
        ),
    );
}

#[derive(Event, Reflect)]
pub struct SelectHotbarSlot(Id<StructureTemplate>);

#[derive(Event, Reflect)]
pub struct HotbarItemSelected(pub Id<StructureTemplate>);

#[derive(Event, Reflect)]
pub struct HotbarItemDeselected;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct HotbarSelection(pub Option<Id<StructureTemplate>>);

#[derive(Component, Reflect)]
#[reflect(Component)]
struct HotbarAction(Id<StructureTemplate>);

#[derive(Component, Reflect)]
#[reflect(Component)]
struct HotbarShortcut(KeyCode);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(AseAnimation)]
struct HotbarIcon(Id<StructureTemplate>);

fn spawn_hotbar(mut commands: Commands) {
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
                (KeyCode::Digit1, "windmill"),
                (KeyCode::Digit2, "power_pole"),
                (KeyCode::Digit3, "miner"),
                (KeyCode::Digit4, "smelter"),
                (KeyCode::Digit5, "constructor"),
                (KeyCode::Digit6, "merger"),
            ]
            .iter()
            .map(move |(shortcut, structure_id)| {
                (
                    Name::new(format!("Hotbar Slot {:?}", structure_id)),
                    Node {
                        width: Val::Px(64.0),
                        height: Val::Px(64.0),
                        border: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    Pickable::default(),
                    BorderColor(Color::WHITE),
                    HotbarShortcut(*shortcut),
                    HotbarAction(Id::new(*structure_id)),
                    children![(
                        Name::new("Icon"),
                        ImageNode::default(),
                        Pickable::IGNORE,
                        HotbarIcon(Id::new(*structure_id)),
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

    commands.trigger(SelectHotbarSlot(action.0.to_owned()));
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

        commands.trigger(SelectHotbarSlot(action.0.to_owned()));
    }
}

fn on_slot_selected(
    trigger: Trigger<SelectHotbarSlot>,
    mut hotbar_selection: ResMut<HotbarSelection>,
    mut commands: Commands,
) {
    let event = trigger.event();

    if hotbar_selection.0 == Some(event.0.to_owned()) {
        hotbar_selection.0 = None;
        commands.trigger(HotbarItemDeselected);
    } else {
        hotbar_selection.0 = Some(event.0.to_owned());
        commands.trigger(HotbarItemSelected(event.0.to_owned()));
    }
}

fn highlight_selected_slot(
    mut commands: Commands,
    hotbar_slots: Query<(Entity, &HotbarAction)>,
    selection: Res<HotbarSelection>,
) {
    for (entity, slot) in hotbar_slots {
        if selection.0.as_ref().is_some_and(|b| *b == slot.0) {
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

fn update_icon(
    query: Query<(&mut AseAnimation, &HotbarIcon), Changed<HotbarIcon>>,
    asset_server: Res<AssetServer>,
) {
    for (mut animation, icon) in query {
        animation.aseprite = asset_server.load(format!("structures/{}.aseprite", icon.0.value));
        animation.animation = Animation::tag("work").with_speed(0.0);
    }
}
