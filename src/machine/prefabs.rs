use std::time::Duration;

use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    info::Details,
    machine::{
        Machine,
        io::{ItemType, ResourceInput, ResourceOutput},
        power::Powered,
        work::Frequency,
    },
    power::{PowerConsumer, PowerProducer},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Reflect)]
pub enum BuildingType {
    #[default]
    Windmill,
    Miner,
    CoalGenerator,
    Constructor,
}

#[derive(Component)]
#[require(
    Machine,
    Name::new("Windmill"),
    Sprite::from_color(Color::linear_rgb(0.9, 0.9, 0.9), Vec2::splat(64.0)),
    PowerProducer(30.0),
    Powered
)]
pub struct Windmill;

#[derive(Component)]
#[require(
    Machine,
    Name::new("Miner"),
    Sprite::from_color(Color::linear_rgb(0.5, 0.0, 0.0), Vec2::splat(64.0)),
    PowerConsumer(5.0),
    Powered,
    Frequency(Duration::from_secs(10)),
    ResourceOutput(HashMap::from([
        (ItemType::Coal, 60)
    ]))
)]
pub struct Miner;

#[derive(Component)]
#[require(
    Machine,
    Name::new("Coal Generator"),
    Sprite::from_color(Color::linear_rgb(0.0, 0.0, 0.0), Vec2::splat(64.0)),
    PowerProducer(75.0),
    Frequency(Duration::from_secs(60)),
    Powered,
    ResourceInput(HashMap::from([
        (ItemType::Coal, 60)
    ]))
)]
pub struct CoalGenerator;

#[derive(Component)]
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
