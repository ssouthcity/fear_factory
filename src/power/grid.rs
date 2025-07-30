use bevy::prelude::*;
use rand::Rng;

use crate::{
    FactorySystems,
    machine::power::Powered,
    power::{FuseBlown, PowerConsumer, PowerProducer, line::PowerLine},
};

pub fn plugin(app: &mut App) {
    app.register_type::<PowerGrid>();
    app.register_type::<PowerGridComponents>();
    app.register_type::<PowerGridComponentOf>();

    app.add_observer(on_new_grid_node);

    app.add_observer(add_power_grid_indicator)
        .add_systems(Update, color_indicators.in_set(FactorySystems::UI));

    app.add_event::<MergeGrids>();

    app.add_systems(Startup, spawn_power_grid_ui)
        .add_observer(add_new_grid_to_ui)
        .add_observer(remove_grid_from_ui)
        .add_systems(Update, update_power_grid_ui.in_set(FactorySystems::UI));

    app.add_systems(
        Update,
        (
            (reset_power_levels, merge_grids),
            (calculate_power_production, calculate_power_consumption),
            check_for_overload,
        )
            .chain()
            .in_set(FactorySystems::Power),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Name::new("Power Grid"))]
pub struct PowerGrid {
    color: Color,
    power_production: f32,
    power_consumption: f32,
}

/// Indicates that the entity can be connected to an electrical grid
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Pickable)]
pub struct GridNode;

fn on_new_grid_node(trigger: Trigger<OnAdd, GridNode>, mut commands: Commands) {
    let mut rng = rand::rng();

    let grid = commands
        .spawn(PowerGrid {
            color: Color::hsl(rng.random_range(0.0..360.0), 1.0, 0.5),
            ..default()
        })
        .id();

    commands
        .entity(trigger.target())
        .insert(PowerGridComponentOf(grid));

    commands
        .entity(trigger.target())
        .observe(on_grid_node_connect);
}

fn on_grid_node_connect(
    trigger: Trigger<Pointer<DragDrop>>,
    grid_nodes: Query<&GridNode>,
    power_grid_component_of: Query<&PowerGridComponentOf>,
    mut events: EventWriter<MergeGrids>,
    mut commands: Commands,
) {
    let event = trigger.event();

    if !grid_nodes.contains(event.dropped) {
        return;
    }

    let Ok(grid_target) = power_grid_component_of.get(event.target) else {
        return;
    };

    let Ok(grid_dropped) = power_grid_component_of.get(event.dropped) else {
        return;
    };

    commands.spawn(PowerLine(event.target, event.dropped));

    events.write(MergeGrids(grid_target.0, grid_dropped.0));
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[relationship_target(relationship = PowerGridComponentOf)]
pub struct PowerGridComponents(Vec<Entity>);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = PowerGridComponents)]
pub struct PowerGridComponentOf(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = PowerGridIndicatorOf, linked_spawn)]
struct PowerGridIndicator(Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = PowerGridIndicator)]
struct PowerGridIndicatorOf(pub Entity);

fn add_power_grid_indicator(
    trigger: Trigger<OnAdd, PowerGridComponentOf>,
    power_grid_component_ofs: Query<&PowerGridComponentOf>,
    power_grids: Query<&PowerGrid>,
    mut commands: Commands,
) {
    let power_grid_component_of = power_grid_component_ofs.get(trigger.target()).unwrap();

    let power_grid = power_grids.get(power_grid_component_of.0).unwrap();

    commands.spawn((
        Name::new("Power Grid Indicator"),
        ChildOf(trigger.target()),
        PowerGridIndicatorOf(trigger.target()),
        Transform::from_xyz(32.0, 28.0, 2.0),
        Sprite::from_color(power_grid.color, Vec2::splat(16.0)),
    ));
}

fn color_indicators(
    power_grids: Query<&PowerGrid>,
    grid_nodes: Query<(&PowerGridComponentOf, &PowerGridIndicator)>,
    mut sprites: Query<&mut Sprite>,
) {
    for (power_grid_component_of, power_grid_indicator) in grid_nodes {
        let Ok(power_grid) = power_grids.get(power_grid_component_of.0) else {
            warn!("PowerGridComponentOf points to non-existent PowerGrid");
            continue;
        };

        let Ok(ref mut sprite) = sprites.get_mut(power_grid_indicator.0) else {
            warn!("PowerGridIndicator is missing it's sprite component");
            continue;
        };

        sprite.color = power_grid.color;
    }
}

#[derive(Event, Reflect)]
pub struct MergeGrids(pub Entity, pub Entity);

fn merge_grids(
    mut events: EventReader<MergeGrids>,
    power_grid_components: Query<&PowerGridComponents>,
    mut commands: Commands,
) {
    for event in events.read() {
        if event.0 == event.1 {
            continue;
        }

        if let Ok(right_components) = power_grid_components.get(event.1) {
            for entity in right_components.iter() {
                commands
                    .entity(entity)
                    .insert(PowerGridComponentOf(event.0));
            }
        };

        commands.entity(event.1).despawn();
    }
}

#[derive(Component, Default)]
struct PowerGridUI;

#[derive(Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
enum PowerGridUIElements {
    ElementOf(Entity),
    ColorOf(Entity),
    PowerProductionOf(Entity),
    PowerConsumptionOf(Entity),
}

fn spawn_power_grid_ui(mut commands: Commands) {
    commands.spawn((
        Name::new("Power Grid UI"),
        PowerGridUI::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::ZERO,
            right: Val::ZERO,
            padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
            margin: UiRect::all(Val::Px(16.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::WHITE.with_alpha(0.2)),
    ));
}

fn add_new_grid_to_ui(
    trigger: Trigger<OnAdd, PowerGrid>,
    power_grid_ui: Single<Entity, With<PowerGridUI>>,
    mut commands: Commands,
) {
    commands.spawn((
        ChildOf(power_grid_ui.into_inner()),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        PowerGridUIElements::ElementOf(trigger.target()),
        children![
            (
                PowerGridUIElements::ColorOf(trigger.target()),
                Node {
                    width: Val::Px(8.0),
                    height: Val::Px(8.0),
                    ..default()
                },
                BackgroundColor(Color::BLACK),
            ),
            (
                PowerGridUIElements::PowerProductionOf(trigger.target()),
                Text::default()
            ),
            (
                PowerGridUIElements::PowerConsumptionOf(trigger.target()),
                Text::default()
            )
        ],
    ));
}

fn remove_grid_from_ui(
    trigger: Trigger<OnRemove, PowerGrid>,
    mut commands: Commands,
    power_grid_ui_elements: Query<(Entity, &PowerGridUIElements)>,
) {
    for (element, power_grid_ui_of) in power_grid_ui_elements {
        if *power_grid_ui_of == PowerGridUIElements::ElementOf(trigger.target()) {
            commands.entity(element).despawn();
        }
    }
}

fn update_power_grid_ui(
    power_grids: Query<&PowerGrid>,
    power_grid_ui_elements: Query<(Entity, &PowerGridUIElements)>,
    mut background_colors: Query<&mut BackgroundColor>,
    mut texts: Query<&mut Text>,
) {
    for (entity, power_grid_ui_element) in power_grid_ui_elements {
        match power_grid_ui_element {
            PowerGridUIElements::ColorOf(grid) => {
                let Ok(power_grid) = power_grids.get(*grid) else {
                    continue;
                };

                let Ok(ref mut background_color) = background_colors.get_mut(entity) else {
                    continue;
                };

                background_color.0 = power_grid.color;
            }
            PowerGridUIElements::PowerProductionOf(grid) => {
                let Ok(power_grid) = power_grids.get(*grid) else {
                    continue;
                };

                let Ok(ref mut text) = texts.get_mut(entity) else {
                    continue;
                };

                text.0 = power_grid.power_production.to_string();
            }
            PowerGridUIElements::PowerConsumptionOf(grid) => {
                let Ok(power_grid) = power_grids.get(*grid) else {
                    continue;
                };

                let Ok(ref mut text) = texts.get_mut(entity) else {
                    continue;
                };

                text.0 = power_grid.power_consumption.to_string();
            }
            PowerGridUIElements::ElementOf(_) => continue,
        }
    }
}

fn reset_power_levels(power_grids: Query<&mut PowerGrid>) {
    for mut grid in power_grids {
        grid.power_production = 0.0;
        grid.power_consumption = 0.0;
    }
}

fn calculate_power_production(
    power_producers: Query<(&PowerProducer, &PowerGridComponentOf), With<Powered>>,
    mut power_grids: Query<&mut PowerGrid>,
) {
    for (power_producer, power_grid_component_of) in power_producers {
        let Ok(mut power_grid) = power_grids.get_mut(power_grid_component_of.0) else {
            continue;
        };

        power_grid.power_production += power_producer.0;
    }
}

fn calculate_power_consumption(
    power_consumers: Query<(&PowerConsumer, &PowerGridComponentOf), With<Powered>>,
    mut power_grids: Query<&mut PowerGrid>,
) {
    for (power_consumer, power_grid_component_of) in power_consumers {
        let Ok(mut power_grid) = power_grids.get_mut(power_grid_component_of.0) else {
            continue;
        };

        power_grid.power_consumption += power_consumer.0;
    }
}

fn check_for_overload(power_grids: Query<(Entity, &PowerGrid)>, mut commands: Commands) {
    for (entity, grid) in power_grids {
        if grid.power_consumption > grid.power_production {
            commands.trigger_targets(FuseBlown, entity);
        }
    }
}
