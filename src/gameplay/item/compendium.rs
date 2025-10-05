use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::gameplay::item::assets::ItemDef;

pub fn plugin(app: &mut App) {
    app.init_state::<CompendiumState>();

    app.add_systems(
        Update,
        toggle_compendium_state.run_if(input_just_pressed(KeyCode::KeyI)),
    );

    app.add_systems(OnEnter(CompendiumState::Item), spawn_item_compendium);
}

#[derive(States, Reflect, Hash, Debug, Clone, PartialEq, Eq, Default)]
enum CompendiumState {
    #[default]
    Closed,
    Item,
}

fn toggle_compendium_state(
    state: Res<State<CompendiumState>>,
    mut next_state: ResMut<NextState<CompendiumState>>,
) {
    match state.get() {
        CompendiumState::Closed => next_state.set(CompendiumState::Item),
        CompendiumState::Item => next_state.set(CompendiumState::Closed),
    }
}

fn spawn_item_compendium(mut commands: Commands, items: Res<Assets<ItemDef>>) {
    let mut items_sorted: Vec<_> = items.iter().collect();
    items_sorted.sort_by(|a, b| a.1.name.cmp(&b.1.name));

    let container = commands
        .spawn((
            Name::new("Item Compendium"),
            DespawnOnExit(CompendiumState::Item),
            Node {
                width: percent(100.0),
                height: percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .id();

    let item_list = commands
        .spawn((
            Node {
                padding: px(16.0).all(),
                display: Display::Grid,
                grid_template_columns: vec![RepeatedGridTrack::auto(2)],
                column_gap: px(8.0),
                row_gap: px(8.0),
                ..default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.5)),
            ChildOf(container),
        ))
        .id();

    for (_, item_def) in items_sorted.iter() {
        let item_box = commands
            .spawn((
                Node {
                    display: Display::Grid,
                    grid_template_columns: vec![
                        RepeatedGridTrack::px(1, 64.0),
                        RepeatedGridTrack::px(1, 256.0),
                    ],
                    grid_template_rows: vec![RepeatedGridTrack::px(2, 32.0)],
                    justify_items: JustifyItems::Start,
                    align_items: AlignItems::Center,
                    column_gap: px(16.0),
                    padding: px(4.0).all(),
                    ..default()
                },
                ChildOf(item_list),
                BackgroundColor(Color::BLACK),
            ))
            .id();

        commands.spawn((
            Node {
                justify_self: JustifySelf::Center,
                grid_column: GridPlacement::span(1),
                grid_row: GridPlacement::span(2),
                width: px(56.0),
                height: px(56.0),
                ..default()
            },
            // children![stack_icon(
            // asset_server.get_id_handle(*item_asset_id).unwrap()
            // )],
            ChildOf(item_box),
        ));

        commands.spawn((
            Text::new(item_def.name.to_owned()),
            TextColor(Color::WHITE),
            ChildOf(item_box),
        ));

        commands.spawn((
            Text::new(item_def.stack_size.to_string()),
            TextColor(Color::WHITE),
            ChildOf(item_box),
        ));
    }
}
