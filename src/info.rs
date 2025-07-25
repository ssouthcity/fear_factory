use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    prelude::*,
};

use crate::{
    machine::{
        frequency::Frequency,
        power::{Powered, TogglePower},
    },
    power::{PowerConsumer, PowerProducer},
};

pub fn plugin(app: &mut App) {
    app.register_type::<Details>();

    app.add_observer(on_detail_insert);

    app.init_resource::<SelectedMachine>();

    app.add_systems(Startup, spawn_details_pane);

    app.add_systems(
        Update,
        (
            update_details_pane_info,
            show_hide_details_pane.run_if(resource_changed::<SelectedMachine>),
        ),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Details;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct DetailsPane;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct SelectedMachine(Option<Entity>);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum DetailContent {
    Name,
    PowerState,
    Frequency,
    PowerProduction,
    PowerConsumption,
}

fn on_detail_insert(trigger: Trigger<OnInsert, Details>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert(Pickable::default())
        .observe(select_machine);
}

fn select_machine(trigger: Trigger<Pointer<Click>>, mut selected_machine: ResMut<SelectedMachine>) {
    selected_machine.0 = Some(trigger.target());
}

fn spawn_details_pane(mut commands: Commands) {
    commands.spawn((
        Name::new("Details Pane"),
        DetailsPane,
        Visibility::Hidden,
        Node {
            width: Val::Px(256.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            right: Val::ZERO,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        },
        BackgroundColor(Color::BLACK),
        children![
            (
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (
                        Text::default(),
                        children![
                            TextSpan::new("Name: "),
                            (TextSpan::default(), DetailContent::Name),
                        ]
                    ),
                    (
                        Node::default(),
                        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
                            parent
                                .spawn((
                                    Button::default(),
                                    children![(
                                        Text::default(),
                                        children![(TextSpan::default(), DetailContent::PowerState)],
                                    )],
                                ))
                                .observe(send_power_toggle);
                        }))
                    )
                ]
            ),
            (
                Text::default(),
                children![
                    TextSpan::new("Frequency: "),
                    (TextSpan::default(), DetailContent::Frequency),
                    TextSpan::new("/s"),
                ]
            ),
            (
                Text::default(),
                children![
                    TextSpan::new("Power Production: "),
                    (TextSpan::default(), DetailContent::PowerProduction),
                    TextSpan::new("/s"),
                ]
            ),
            (
                Text::default(),
                children![
                    TextSpan::new("Power Consumption: "),
                    (TextSpan::default(), DetailContent::PowerConsumption),
                    TextSpan::new("/s"),
                ]
            )
        ],
    ));
}

fn update_details_pane_info(
    selected_machine: Res<SelectedMachine>,
    details_spans: Query<(&mut TextSpan, &DetailContent)>,
    name_query: Query<&Name>,
    power_state_query: Query<&Powered>,
    frequency_query: Query<&Frequency>,
    power_production_query: Query<&PowerProducer>,
    power_consumption_query: Query<&PowerConsumer>,
) {
    let Some(machine) = selected_machine.0 else {
        return;
    };

    for (mut span, content) in details_spans {
        span.0 = match content {
            DetailContent::Name => name_query
                .get(machine)
                .map(|n| n.to_string())
                .unwrap_or("N/A".to_owned()),

            DetailContent::PowerState => power_state_query
                .get(machine)
                .map(|_| "On")
                .unwrap_or("Off")
                .to_string(),

            DetailContent::Frequency => frequency_query
                .get(machine)
                .map(|f| f.0.as_secs_f32().to_string())
                .unwrap_or("N/A".to_owned()),

            DetailContent::PowerProduction => power_production_query
                .get(machine)
                .map(|c| c.0.to_string())
                .unwrap_or("N/A".to_owned()),

            DetailContent::PowerConsumption => power_consumption_query
                .get(machine)
                .map(|c| c.0.to_string())
                .unwrap_or("N/A".to_owned()),
        };
    }
}

fn show_hide_details_pane(
    mut pane: Single<&mut Visibility, With<DetailsPane>>,
    selected_machine: Res<SelectedMachine>,
) {
    **pane = if selected_machine.0.is_some() {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}

fn send_power_toggle(
    _trigger: Trigger<Pointer<Click>>,
    selected_machine: Res<SelectedMachine>,
    mut commands: Commands,
) {
    if let Some(machine) = selected_machine.0 {
        commands.trigger_targets(TogglePower, machine);
    };
}
