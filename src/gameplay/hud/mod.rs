use bevy::{
    input_focus::{InputDispatchPlugin, tab_navigation::TabNavigationPlugin},
    prelude::*,
    ui_widgets::{RadioButton, RadioGroup, UiWidgetsPlugins, ValueChange, observe},
};
use bevy_aseprite_ultra::prelude::*;

use crate::screens::Screen;

pub mod hotbar;
pub mod inspect;

pub fn plugin(app: &mut App) {
    app.add_plugins((UiWidgetsPlugins, InputDispatchPlugin, TabNavigationPlugin));

    app.add_plugins((hotbar::plugin, inspect::plugin));

    app.init_state::<ChyronTab>();

    app.add_systems(
        OnEnter(Screen::Gameplay),
        (setup_portrait, setup_chyron, setup_relic),
    );

    app.add_systems(
        Update,
        style_chyron_tab_radio.run_if(state_changed::<ChyronTab>),
    );

    app.add_systems(OnEnter(ChyronTab::Build), spawn_build_menu);
    app.add_systems(OnEnter(ChyronTab::People), spawn_people_menu);
    app.add_systems(OnEnter(ChyronTab::Info), spawn_info_menu);
}

fn setup_portrait(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: vw(15.0),
            height: auto(),
            position_type: PositionType::Absolute,
            bottom: percent(0.0),
            left: percent(0.0),
            ..default()
        },
        ImageNode::default(),
        AseAnimation {
            aseprite: asset_server.load("sprites/hud/player_portrait.aseprite"),
            animation: Animation::tag("thinking"),
        },
    ));
}

fn setup_relic(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: vw(15.0),
            height: auto(),
            position_type: PositionType::Absolute,
            bottom: percent(0.0),
            right: percent(0.0),
            ..default()
        },
        ImageNode::default(),
        AseAnimation {
            aseprite: asset_server.load("sprites/hud/hand.aseprite"),
            animation: Animation::tag("fidget"),
        },
    ));
}

#[derive(States, Component, Reflect, Hash, Clone, Copy, PartialEq, Eq, Debug, Default)]
#[reflect(Component)]
pub enum ChyronTab {
    #[default]
    Build,
    People,
    Info,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct ChyronContent;

fn setup_chyron(mut commands: Commands) {
    commands.spawn((
        Name::new("Chyron"),
        Node {
            width: vw(60.0),
            height: auto(),
            position_type: PositionType::Absolute,
            left: vw(20.0),
            bottom: percent(0.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        children![chyron_tab_menu(), chyron_content_container()],
    ));
}

fn chyron_tab_menu() -> impl Bundle {
    (
        Name::new("Tabs"),
        RadioGroup,
        Node {
            display: Display::Flex,
            column_gap: px(16.0),
            ..default()
        },
        observe(
            |value_change: On<ValueChange<Entity>>,
             mut next_chyron_state: ResMut<NextState<ChyronTab>>,
             q_radios: Query<&ChyronTab>| {
                if let Ok(tab) = q_radios.get(value_change.value) {
                    next_chyron_state.set(*tab);
                }
            },
        ),
        children![
            (
                Text::new("Build"),
                RadioButton,
                ChyronTab::Build,
                BackgroundColor(Color::hsl(120.0, 1.0, 0.5))
            ),
            (Text::new("People"), RadioButton, ChyronTab::People),
            (Text::new("Info"), RadioButton, ChyronTab::Info),
        ],
    )
}

fn chyron_content_container() -> impl Bundle {
    (
        Name::new("Content"),
        ChyronContent,
        Node {
            padding: px(8.0).all(),
            ..default()
        },
    )
}

fn style_chyron_tab_radio(
    state: Res<State<ChyronTab>>,
    q_radios: Query<(Entity, &ChyronTab)>,
    mut commands: Commands,
) {
    let current_tab = state.get();

    for (radio, radio_value) in q_radios {
        if current_tab == radio_value {
            commands
                .entity(radio)
                .insert(BackgroundColor(Color::hsl(120.0, 1.0, 0.5)));
        } else {
            commands.entity(radio).remove::<BackgroundColor>();
        }
    }
}

fn spawn_build_menu(mut commands: Commands, container: Single<Entity, With<ChyronContent>>) {
    commands.entity(*container).despawn_children();

    commands.spawn((
        Name::new("Build Menu"),
        ChildOf(*container),
        card_display(),
        children![card("constructor"), card("harvester"), card("road"),],
    ));
}

fn card_display() -> impl Bundle {
    (Node {
        display: Display::Flex,
        column_gap: px(16.0),
        ..default()
    },)
}

fn card(name: &'static str) -> impl Bundle {
    (
        Name::new(name),
        Node {
            width: px(64.0),
            height: px(96.0),
            overflow: Overflow::hidden(),
            ..default()
        },
        BackgroundColor(Color::hsl(240.0, 1.0, 0.5)),
        BorderRadius::all(px(8.0)),
        children![Text::new(name)],
    )
}

fn spawn_people_menu(mut commands: Commands, container: Single<Entity, With<ChyronContent>>) {
    commands.entity(*container).despawn_children();

    commands.spawn((
        Name::new("People Menu"),
        ChildOf(*container),
        card_display(),
        children![card("bob"), card("alice")],
    ));
}

fn spawn_info_menu(mut commands: Commands, container: Single<Entity, With<ChyronContent>>) {
    commands.entity(*container).despawn_children();

    commands.spawn((
        Name::new("Info Menu"),
        ChildOf(*container),
        Text::new("Konichiwagwan mina-slimes"),
    ));
}
