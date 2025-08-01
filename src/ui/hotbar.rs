use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::{FactorySystems, build::Buildable};

pub fn plugin(app: &mut App) {
    app.register_type::<HotbarSelection>();
    app.register_type::<HotbarSlot>();

    app.init_resource::<HotbarSelection>();

    app.add_systems(Startup, spawn_hotbar);

    app.add_systems(Update, highlight_selected_slot.in_set(FactorySystems::UI));
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct HotbarSelection(pub Buildable);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Pickable)]
struct HotbarSlot(Buildable);

fn spawn_hotbar(mut commands: Commands, asset_server: Res<AssetServer>) {
    let aseprite = asset_server.load::<Aseprite>("build-icons.aseprite");

    let mut hotbar_observer = Observer::new(on_build_hotbar_click);

    let hotbar = commands
        .spawn((
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
        ))
        .id();

    for (building, slice_name) in [
        (Buildable::Windmill, "Windmill"),
        (Buildable::PowerPole, "Power Pole"),
        (Buildable::Miner, "Miner"),
        (Buildable::CoalGenerator, "Coal Generator"),
        (Buildable::Constructor, "Constructor"),
    ] {
        let id = commands
            .spawn((
                Name::new(format!("Hotbar Slot {:?}", building)),
                Node {
                    width: Val::Px(64.0),
                    height: Val::Px(64.0),
                    border: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                BorderColor(Color::WHITE),
                HotbarSlot(building.clone()),
                children![(
                    Name::new("Icon"),
                    ImageNode::default(),
                    AseSlice {
                        aseprite: aseprite.clone(),
                        name: slice_name.to_string(),
                    },
                )],
            ))
            .id();

        commands.entity(hotbar).add_child(id);
        hotbar_observer.watch_entity(id);
    }

    commands.spawn(hotbar_observer);
}

fn on_build_hotbar_click(
    trigger: Trigger<Pointer<Click>>,
    hotbar_slots: Query<&HotbarSlot>,
    mut hotbar_selection: ResMut<HotbarSelection>,
) {
    if let Ok(slot) = hotbar_slots.get(trigger.target()) {
        hotbar_selection.0 = slot.0;
    }
}

fn highlight_selected_slot(
    mut commands: Commands,
    hotbar_slots: Query<(Entity, &HotbarSlot)>,
    selection: Res<HotbarSelection>,
) {
    for (entity, slot) in hotbar_slots {
        if slot.0 == selection.0 {
            commands
                .entity(entity)
                .insert(BackgroundColor(Color::WHITE));
        } else {
            commands.entity(entity).remove::<BackgroundColor>();
        }
    }
}
