use std::time::Duration;

use bevy::prelude::*;

use crate::{
    animation::AnimatedMachine,
    info::Details,
    logistics::{ItemCollection, ItemID, ResourceInput, ResourceOutput},
    machine::{
        Machine,
        work::{Frequency, Working},
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
    Working::default(),
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
    Frequency(Duration::from_secs(10)),
    ResourceOutput(ItemCollection::new().with_item(ItemID::Coal, 30)),
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
    ResourceInput(ItemCollection::new().with_item(ItemID::Coal, 60)),
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
    PowerSockets::single()
)]
pub struct Constructor;
