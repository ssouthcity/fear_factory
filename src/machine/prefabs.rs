use std::time::Duration;

use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    animation::AnimatedMachine,
    info::Details,
    machine::{
        Machine,
        io::{ItemType, ResourceInput, ResourceOutput},
        power::Powered,
        work::Frequency,
    },
    power::{PowerConsumer, PowerProducer, socket::PowerSockets},
};

#[derive(Component)]
#[require(
    Machine,
    Name::new("Windmill"),
    Sprite::sized(Vec2::splat(64.0)),
    AnimatedMachine("windmill.aseprite"),
    PowerProducer(30.0),
    Powered,
    PowerSockets::single()
)]
pub struct Windmill;

#[derive(Component)]
#[require(
    Machine,
    Name::new("Miner"),
    Sprite::sized(Vec2::splat(64.0)),
    AnimatedMachine("miner.aseprite"),
    PowerConsumer(5.0),
    Powered,
    Frequency(Duration::from_secs(10)),
    ResourceOutput(HashMap::from([
        (ItemType::Coal, 60)
    ])),
    PowerSockets::single(),
)]
pub struct Miner;

#[derive(Component)]
#[require(
    Machine,
    Name::new("Coal Generator"),
    Sprite::sized(Vec2::splat(64.0)),
    AnimatedMachine("coal-generator.aseprite"),
    PowerProducer(75.0),
    Frequency(Duration::from_secs(60)),
    Powered,
    ResourceInput(HashMap::from([
        (ItemType::Coal, 60)
    ])),
    PowerSockets::single()
)]
pub struct CoalGenerator;

#[derive(Component)]
#[require(
    Machine,
    Name::new("Constructor"),
    Sprite::sized(Vec2::splat(64.0)),
    AnimatedMachine("constructor.aseprite"),
    PowerConsumer(15.0),
    Frequency(Duration::from_secs(3)),
    Details,
    Powered,
    PowerSockets::single()
)]
pub struct Constructor;
