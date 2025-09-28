use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{gameplay::item::assets::ItemDef, widgets::item::item_icon};

pub fn plugin(app: &mut App) {
    app.register_type::<CompendiumState>();
    app.init_state::<CompendiumState>();

    app.add_systems(
        Update,
        toggle_compendium_state.run_if(input_just_pressed(KeyCode::KeyI)),
    );

    app.add_systems(OnEnter(CompendiumState::Item), spawn_item_compendium);
}

#[derive(States, Reflect, Hash, Debug, Clone, PartialEq, Eq, Default)]
#[states(scoped_entities)]
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

fn spawn_item_compendium(
    mut commands: Commands,
    items: Res<Assets<ItemDef>>,
    asset_server: Res<AssetServer>,
) {
    let mut items_sorted: Vec<_> = items.iter().collect();
    items_sorted.sort_by(|a, b| a.1.name.cmp(&b.1.name));

    let container = commands
        .spawn((
            Name::new("Item Compendium"),
            StateScoped(CompendiumState::Item),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .id();

    let item_list = commands
        .spawn((
            Node {
                padding: UiRect::all(Val::Px(16.0)),
                display: Display::Grid,
                grid_template_columns: vec![RepeatedGridTrack::auto(2)],
                column_gap: Val::Px(8.0),
                row_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.5)),
            ChildOf(container),
        ))
        .id();

    for (item_asset_id, item_def) in items_sorted.iter() {
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
                    column_gap: Val::Px(16.0),
                    padding: UiRect::all(Val::Px(4.0)),
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
                width: Val::Px(56.0),
                height: Val::Px(56.0),
                ..default()
            },
            children![item_icon(
                asset_server.get_id_handle(*item_asset_id).unwrap()
            )],
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
