use bevy::{
    ecs::{spawn::SpawnIter, system::SystemParam},
    prelude::*,
};
use bevy_aseprite_ultra::prelude::*;

use crate::{
    gameplay::{FactorySystems, structure::assets::StructureDef, world::tilemap::TileClicked},
    input::input_map::{Action, InputActions},
    screens::Screen,
};

const HOTBAR_ACTIONS: [Action; 9] = [
    Action::Hotbar1,
    Action::Hotbar2,
    Action::Hotbar3,
    Action::Hotbar4,
    Action::Hotbar5,
    Action::Hotbar6,
    Action::Hotbar7,
    Action::Hotbar8,
    Action::Hotbar9,
];

pub fn plugin(app: &mut App) {
    app.init_resource::<HotbarSelectedEntity>();

    app.add_message::<HotbarSelectionChanged>();

    app.add_systems(
        OnEnter(Screen::Gameplay),
        (spawn_hotbar, assign_hotbar_structures, assign_hotbar_paths).chain(),
    );

    app.add_observer(select_on_click);

    app.add_systems(
        Update,
        (select_on_keyboard_shortcuts, highlight_selected_slot),
    );

    app.add_systems(
        FixedPostUpdate,
        deselect_on_use
            .run_if(on_message::<TileClicked>)
            .after(FactorySystems::Construction),
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
    hotbar_selection_changes: MessageWriter<'w, HotbarSelectionChanged>,
}

impl HotbarSelector<'_> {
    fn select(&mut self, entity: Option<Entity>) {
        if self.selection.0 == entity {
            if self.selection.is_some() {
                self.hotbar_selection_changes.write(HotbarSelectionChanged {
                    previous: self.selection.0,
                    current: None,
                });
                self.selection.0 = None;
            }
        } else {
            self.hotbar_selection_changes.write(HotbarSelectionChanged {
                previous: self.selection.0,
                current: entity,
            });
            self.selection.0 = entity;
        }
    }
}

#[derive(Message, Reflect, Debug)]
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
struct HotbarShortcut(Action);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct HotbarPersistAfterUse;

fn spawn_hotbar(mut commands: Commands) {
    commands.spawn((
        Name::new("Hotbar"),
        DespawnOnExit(Screen::Gameplay),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(8.0),
            width: auto(),
            height: auto(),
            margin: auto().horizontal(),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: px(8.0),
            row_gap: px(8.0),
            ..default()
        },
        Pickable::IGNORE,
        Children::spawn(SpawnIter((0..HOTBAR_ACTIONS.len()).map(|i| {
            (
                Name::new(format!("Hotbar Slot {}", i + 1)),
                Node {
                    width: px(64.0),
                    height: px(64.0),
                    border: px(4.0).all(),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Pickable::default(),
                BorderColor::all(Color::WHITE),
                HotbarSlot,
                HotbarShortcut(HOTBAR_ACTIONS[i]),
            )
        }))),
    ));
}

fn assign_hotbar_structures(
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
            HotbarActionKind::PlaceStructure(asset_server.get_id_handle(asset_id).unwrap()),
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

fn assign_hotbar_paths(
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
        HotbarPersistAfterUse,
        Pickable::IGNORE,
        Node::default(),
        children![(
            ImageNode::default(),
            AseSlice {
                aseprite: asset_server.load("sprites/logistics/path_segments.aseprite"),
                name: "C".into(),
            },
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
    pointer_click: On<Pointer<Click>>,
    hotbar_slots: Query<Entity, With<HotbarSlot>>,
    mut hotbar: HotbarSelector,
) {
    if hotbar_slots.contains(pointer_click.entity) {
        hotbar.select(Some(pointer_click.entity));
    };
}

fn select_on_keyboard_shortcuts(
    hotbar_slots: Query<(Entity, &HotbarShortcut), With<HotbarSlot>>,
    input_actions: Res<InputActions>,
    mut hotbar: HotbarSelector,
) {
    for (entity, shortcut) in hotbar_slots {
        if input_actions.just_pressed.contains(&shortcut.0) {
            hotbar.select(Some(entity));
        }
    }
}

fn deselect_on_use(
    mut selector: HotbarSelector,
    hotbar_actions: Query<&HotbarAction>,
    persisted_actions: Query<Entity, With<HotbarPersistAfterUse>>,
) {
    let Some(selected) = selector.selection.0 else {
        return;
    };

    let Ok(HotbarAction(action)) = hotbar_actions.get(selected) else {
        return;
    };

    if persisted_actions.contains(*action) {
        return;
    }

    selector.select(None);
}
