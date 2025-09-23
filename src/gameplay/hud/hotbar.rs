use bevy::{
    ecs::{spawn::SpawnIter, system::SystemParam},
    input::keyboard::KeyboardInput,
    prelude::*,
};
use bevy_aseprite_ultra::prelude::*;

use crate::{
    gameplay::{
        FactorySystems, structure::assets::StructureDef, world::construction::StructureConstructed,
    },
    screens::Screen,
};

const DIGIT_KEYS: [KeyCode; 9] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
    KeyCode::Digit6,
    KeyCode::Digit7,
    KeyCode::Digit8,
    KeyCode::Digit9,
];

pub fn plugin(app: &mut App) {
    app.register_type::<HotbarSlot>();
    app.register_type::<HotbarAction>();
    app.register_type::<HotbarActionOf>();
    app.register_type::<HotbarActionKind>();
    app.register_type::<HotbarShortcut>();

    app.register_type::<HotbarSelectedEntity>();
    app.init_resource::<HotbarSelectedEntity>();

    app.add_event::<HotbarSelectionChanged>();

    app.add_systems(
        OnEnter(Screen::Gameplay),
        (spawn_hotbar, assign_hotbar_items, assign_path_action).chain(),
    );

    app.add_observer(select_on_click);

    app.add_systems(
        Update,
        (
            select_on_keyboard_shortcuts
                .in_set(FactorySystems::Input)
                .run_if(on_event::<KeyboardInput>),
            highlight_selected_slot.in_set(FactorySystems::UI),
            deselect_slot.run_if(on_event::<StructureConstructed>),
        ),
    );
}

#[derive(SystemParam)]
pub struct HotbarSelection<'w, 's> {
    hotbar_selected_entity: Res<'w, HotbarSelectedEntity>,
    hotbar_actions: Query<'w, 's, &'static HotbarAction>,
    hotbar_action_kind: Query<'w, 's, &'static HotbarActionKind>,
}

impl HotbarSelection<'_, '_> {
    pub fn action(&self) -> Option<&HotbarActionKind> {
        self.hotbar_selected_entity
            .and_then(|selection| self.hotbar_actions.get(selection).ok())
            .and_then(|action| self.hotbar_action_kind.get(action.0).ok())
    }
}

#[derive(SystemParam)]
struct HotbarSelector<'w> {
    selection: ResMut<'w, HotbarSelectedEntity>,
    events: EventWriter<'w, HotbarSelectionChanged>,
}

impl HotbarSelector<'_> {
    fn select(&mut self, entity: Option<Entity>) {
        if self.selection.0 == entity {
            if self.selection.is_some() {
                self.events.write(HotbarSelectionChanged {
                    previous: self.selection.0,
                    current: None,
                });
                self.selection.0 = None;
            }
        } else {
            self.events.write(HotbarSelectionChanged {
                previous: self.selection.0,
                current: entity,
            });
            self.selection.0 = entity;
        }
    }
}

#[derive(Event, Reflect, Debug)]
pub struct HotbarSelectionChanged {
    pub previous: Option<Entity>,
    pub current: Option<Entity>,
}

#[derive(Resource, Reflect, Debug, Default, Deref, DerefMut)]
#[reflect(Resource)]
struct HotbarSelectedEntity(Option<Entity>);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
struct HotbarSlot;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship_target(relationship = HotbarActionOf, linked_spawn)]
struct HotbarAction(Entity);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[relationship(relationship_target = HotbarAction)]
struct HotbarActionOf(pub Entity);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub enum HotbarActionKind {
    PlaceStructure(Handle<StructureDef>),
    PlacePath,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct HotbarShortcut(KeyCode);

fn spawn_hotbar(mut commands: Commands) {
    commands.spawn((
        Name::new("Hotbar"),
        StateScoped(Screen::Gameplay),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(8.0),
            width: Val::Auto,
            height: Val::Auto,
            margin: UiRect::axes(Val::Auto, Val::ZERO),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.0),
            row_gap: Val::Px(8.0),
            ..default()
        },
        Pickable::IGNORE,
        Children::spawn(SpawnIter((0..DIGIT_KEYS.len()).map(|i| {
            (
                Name::new(format!("Hotbar Slot {}", i + 1)),
                Node {
                    width: Val::Px(64.0),
                    height: Val::Px(64.0),
                    border: UiRect::all(Val::Px(4.0)),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Pickable::default(),
                BorderColor(Color::WHITE),
                HotbarSlot,
                HotbarShortcut(DIGIT_KEYS[i]),
            )
        }))),
    ));
}

fn assign_hotbar_items(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    structure_defs: Res<Assets<StructureDef>>,
    query: Query<Entity, (With<HotbarShortcut>, Without<Children>)>,
) {
    for (hotbar_slot, (asset_id, structure_def)) in query.iter().zip(structure_defs.iter()) {
        commands.spawn((
            Name::new("Hotbar Action"),
            ChildOf(hotbar_slot),
            HotbarActionOf(hotbar_slot),
            HotbarActionKind::PlaceStructure(Handle::Weak(asset_id)),
            Pickable::IGNORE,
            Node::default(),
            children![(
                ImageNode::default(),
                AseAnimation {
                    aseprite: asset_server
                        .load(format!("sprites/structures/{}.aseprite", structure_def.id)),
                    animation: Animation::tag("work").with_speed(0.0),
                },
                Pickable::IGNORE,
            )],
        ));
    }
}

fn assign_path_action(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, (With<HotbarShortcut>, Without<Children>)>,
) {
    let Some(hotbar_slot) = query.iter().next() else {
        return;
    };

    commands.spawn((
        Name::new("Hotbar Action"),
        ChildOf(hotbar_slot),
        HotbarActionOf(hotbar_slot),
        HotbarActionKind::PlacePath,
        Pickable::IGNORE,
        Node::default(),
        children![(
            ImageNode::new(asset_server.load("sprites/logistics/path.png")),
            Pickable::IGNORE,
        )],
    ));
}

fn highlight_selected_slot(
    mut commands: Commands,
    current_selection: Res<HotbarSelectedEntity>,
    mut highlighted_selection: Local<Option<Entity>>,
) {
    if let Some(highlighted) = *highlighted_selection {
        commands.entity(highlighted).remove::<BackgroundColor>();
    }

    if let Some(selection) = current_selection.0 {
        commands
            .entity(selection)
            .insert(BackgroundColor(Color::WHITE));

        *highlighted_selection = Some(selection);
    }
}

fn select_on_click(
    trigger: Trigger<Pointer<Click>>,
    hotbar_slots: Query<Entity, With<HotbarSlot>>,
    mut hotbar: HotbarSelector,
) {
    if hotbar_slots.contains(trigger.target) {
        hotbar.select(Some(trigger.target));
    };
}

fn select_on_keyboard_shortcuts(
    keys: Res<ButtonInput<KeyCode>>,
    hotbar_slots: Query<(Entity, &HotbarShortcut), With<HotbarSlot>>,
    mut hotbar: HotbarSelector,
) {
    for (entity, shortcut) in hotbar_slots {
        if keys.just_pressed(shortcut.0) {
            hotbar.select(Some(entity));
        }
    }
}

fn deselect_slot(mut hotbar: HotbarSelector) {
    hotbar.select(None);
}
