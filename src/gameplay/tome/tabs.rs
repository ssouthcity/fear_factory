use bevy::{prelude::*, state::state::FreelyMutableState, ui::Checked, ui_widgets::observe};

use crate::gameplay::tome::constants;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_tab_color);
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(super) struct Tab;

pub(super) fn tab<T: Component + FreelyMutableState>(
    asset_server: &AssetServer,
    name: &'static str,
    value: T,
) -> impl Bundle {
    (
        Name::new(name),
        Node {
            padding: UiRect::axes(px(16.0), px(12.0)),
            ..default()
        },
        Tab,
        ImageNode {
            image: asset_server.load("sprites/hud/bookmark.png"),
            image_mode: NodeImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(12.0),
                sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
                ..default()
            }),
            color: constants::TAB_COLOR_DEFAULT,
            ..default()
        },
        Interaction::default(),
        value,
        observe(on_tab_select::<T>),
        children![Text::new(name),],
    )
}

fn update_tab_color(tabs: Query<(&mut ImageNode, Option<&Interaction>, Has<Checked>), With<Tab>>) {
    for (mut image_node, interaction, checked) in tabs {
        let hovered_or_pressed = matches!(
            interaction,
            Some(Interaction::Hovered | Interaction::Pressed)
        );

        image_node.color = if checked {
            constants::TAB_COLOR_CHECKED
        } else if hovered_or_pressed {
            constants::TAB_COLOR_DEFAULT_HOVERED
        } else {
            constants::TAB_COLOR_DEFAULT
        };
    }
}

fn on_tab_select<T: Component + FreelyMutableState>(
    click: On<Pointer<Click>>,
    tabs: Query<&T>,
    mut next_state: ResMut<NextState<T>>,
) {
    if let Ok(tab) = tabs.get(click.entity) {
        next_state.set(tab.clone());
    }
}

pub(super) fn sync_tab_checked<T: States + Component>(
    tabs: Query<(Entity, &T), With<Tab>>,
    state: Res<State<T>>,
    mut commands: Commands,
) {
    for (tab, tab_state) in tabs {
        if tab_state == state.get() {
            commands.entity(tab).insert(Checked);
        } else {
            commands.entity(tab).remove::<Checked>();
        }
    }
}
