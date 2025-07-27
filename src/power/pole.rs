use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};

use crate::power::grid::{PowerGrid, PowerGridComponentOf};

pub fn plugin(app: &mut App) {
    app.register_type::<PowerPole>();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[component(on_add = on_add_power_pole)]
#[require(
    Name::new("Power Pole"),
    Sprite::from_color(Color::linear_rgb(0.2, 0.2, 0.2), Vec2::new(8.0, 32.0))
)]
pub struct PowerPole;

fn on_add_power_pole(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let grid = world.commands().spawn(PowerGrid).id();

    world
        .commands()
        .entity(entity)
        .insert(PowerGridComponentOf(grid));
}
