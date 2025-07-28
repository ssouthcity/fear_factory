use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<PowerGrid>();
    app.register_type::<PowerLevel>();
    app.register_type::<PowerGridComponents>();
    app.register_type::<PowerGridComponentOf>();

    app.add_observer(on_add_tmp);

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

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[relationship_target(relationship = PowerGridComponentOf)]
pub struct PowerGridComponents(Vec<Entity>);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = PowerGridComponents)]
pub struct PowerGridComponentOf(pub Entity);

fn on_add_tmp(
    trigger: Trigger<OnInsert, PowerGridComponentOf>,
    pgcos: Query<&PowerGridComponentOf>,
    pgs: Query<&PowerGrid>,
    mut sprites: Query<&mut Sprite>,
) {
    let pgco = pgcos.get(trigger.target()).unwrap();

    let color = pgs.get(pgco.0).unwrap();

    if let Ok(ref mut sprite) = sprites.get_mut(trigger.target()) {
        sprite.color = color.0;
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
