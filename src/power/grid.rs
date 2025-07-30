use bevy::prelude::*;
use rand::Rng;

pub fn plugin(app: &mut App) {
    app.register_type::<PowerGrid>();
    app.register_type::<PowerLevel>();
    app.register_type::<PowerGridComponents>();
    app.register_type::<PowerGridComponentOf>();

    app.add_observer(on_new_grid_node);

    app.add_observer(add_power_grid_indicator)
        .add_systems(Update, color_indicators);

    app.add_event::<MergeGrids>();

    app.add_systems(Update, merge_grids);
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Name::new("Power Grid"), PowerLevel)]
pub struct PowerGrid(pub Color);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PowerLevel(pub f32);

/// Indicates that the entity can be connected to an electrical grid
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Pickable)]
pub struct GridNode;

fn on_new_grid_node(trigger: Trigger<OnAdd, GridNode>, mut commands: Commands) {
    let mut rng = rand::rng();

    let grid = commands
        .spawn(PowerGrid(Color::hsl(
            rng.random_range(0.0..360.0),
            1.0,
            0.5,
        )))
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
        Sprite::from_color(power_grid.0, Vec2::splat(16.0)),
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

        sprite.color = power_grid.0;
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
