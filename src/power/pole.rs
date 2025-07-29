use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};
use rand::Rng;

use crate::{
    animation::AnimatedMachine,
    power::grid::{MergeGrids, PowerGrid, PowerGridComponentOf},
};

pub fn plugin(app: &mut App) {
    app.register_type::<PowerPole>();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[component(on_add = on_add_power_pole)]
#[require(
    Name::new("Power Pole"),
    AnimatedMachine("power-pole.aseprite"),
    Sprite::sized(Vec2::splat(64.0)),
    Pickable::default()
)]
pub struct PowerPole;

fn on_add_power_pole(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let mut rng = rand::rng();

    let grid = world
        .commands()
        .spawn(PowerGrid(Color::hsl(
            rng.random_range(0.0..360.0),
            1.0,
            0.5,
        )))
        .id();

    world
        .commands()
        .entity(entity)
        .insert(PowerGridComponentOf(grid));

    world.commands().entity(entity).observe(on_drag_drop);
}

fn on_drag_drop(
    trigger: Trigger<Pointer<DragDrop>>,
    power_poles: Query<&PowerPole>,
    power_grid_component_of: Query<&PowerGridComponentOf>,
    mut events: EventWriter<MergeGrids>,
) {
    let event = trigger.event();

    if !power_poles.contains(event.target) || !power_poles.contains(event.dropped) {
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
