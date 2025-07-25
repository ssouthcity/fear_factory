use std::time::Duration;

use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};

use crate::{
    info::Details,
    machine::{Machine, Work, frequency::Frequency, power::Powered},
    power::{PowerConsumer, PowerProducer},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Reflect)]
pub enum BuildingType {
    #[default]
    Miner,
    CoalGenerator,
    Constructor,
}

#[derive(Component)]
#[require(
    Machine,
    Name::new("Miner"),
    Sprite::from_color(Color::linear_rgb(0.5, 0.0, 0.0), Vec2::splat(64.0)),
    PowerConsumer(10.0),
    Details,
    Powered
)]
pub struct Miner;

#[derive(Component)]
#[require(
    Machine,
    Name::new("Coal Generator"),
    Sprite::from_color(Color::linear_rgb(0.0, 0.0, 0.0), Vec2::splat(64.0)),
    PowerProducer(20.0),
    Frequency(Duration::from_secs(1)),
    Details,
    Powered
)]
pub struct CoalGenerator;

#[derive(Component)]
#[component(
    on_insert = on_constructor_insert
)]
#[require(
    Machine,
    Name::new("Constructor"),
    Sprite::from_color(Color::linear_rgb(0.0, 0.0, 0.5), Vec2::splat(64.0)),
    PowerConsumer(15.0),
    Frequency(Duration::from_secs(3)),
    Details,
    Powered
)]
pub struct Constructor;

fn on_constructor_insert(mut world: DeferredWorld, context: HookContext) {
    world
        .commands()
        .entity(context.entity)
        .observe(on_constructor_work);
}

fn on_constructor_work(_trigger: Trigger<Work>) {
    info!("constructor work!");
}
