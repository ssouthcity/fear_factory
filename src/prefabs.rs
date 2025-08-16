use bevy::prelude::*;

use crate::{
    animation::AnimatedMachine,
    item::SelectedRecipe,
    logistics::{ConveyorHole, ConveyorHoles},
    machine::{Machine, work::Working},
    power::{PowerConsumer, PowerProducer, socket::PowerSockets},
    ui::Interactable,
};

pub fn windmill() -> impl Bundle {
    (
        Name::new("Windmill"),
        Machine,
        Sprite::sized(Vec2::splat(64.0)),
        AnimatedMachine("windmill.aseprite"),
        PowerProducer(30.0),
        Working,
        PowerSockets::single(),
        Interactable,
    )
}

pub fn windmill_preview() -> impl Bundle {
    (
        Name::new("Windmill Preview"),
        Sprite::from_color(Color::WHITE.with_alpha(0.5), Vec2::splat(64.0)),
        AnimatedMachine("windmill.aseprite"),
    )
}

pub fn miner() -> impl Bundle {
    (
        Name::new("Miner"),
        Machine,
        Sprite::sized(Vec2::splat(64.0)),
        AnimatedMachine("miner.aseprite"),
        PowerConsumer(5.0),
        PowerSockets::single(),
        Interactable,
        related!(ConveyorHoles[
            (
                Name::new("Conveyor Hole Outbound"),
                Transform::from_xyz(16.0, 16.0, 10.0),
                ConveyorHole::Outbound,
            ),
        ]),
    )
}

pub fn miner_preview() -> impl Bundle {
    (
        Name::new("Miner Preview"),
        Sprite::from_color(Color::WHITE.with_alpha(0.5), Vec2::splat(64.0)),
        AnimatedMachine("miner.aseprite"),
    )
}

pub fn coal_generator() -> impl Bundle {
    (
        Name::new("Coal Generator"),
        Machine,
        Sprite::sized(Vec2::splat(64.0)),
        AnimatedMachine("coal-generator.aseprite"),
        PowerProducer(75.0),
        PowerSockets::single(),
        Interactable,
        related!(
            ConveyorHoles[(
                Name::new("Conveyor Hole Inbound"),
                Transform::from_xyz(-28.0, -16.0, 10.0),
                ConveyorHole::Inbound
            )]
        ),
    )
}

pub fn coal_generator_preview() -> impl Bundle {
    (
        Name::new("Coal Generator Preview"),
        Sprite::from_color(Color::WHITE.with_alpha(0.5), Vec2::splat(64.0)),
        AnimatedMachine("coal-generator.aseprite"),
    )
}

pub fn constructor() -> impl Bundle {
    (
        Name::new("Constructor"),
        Machine,
        Sprite::sized(Vec2::splat(64.0)),
        AnimatedMachine("constructor.aseprite"),
        PowerConsumer(15.0),
        PowerSockets::single(),
        Interactable,
        SelectedRecipe::default(),
        related!(ConveyorHoles[
            (
                Name::new("Conveyor Hole Inbound"),
                Transform::from_xyz(-18.0, -12.0, 10.0),
                ConveyorHole::Inbound
            ),
            (
                Name::new("Conveyor Hole Outbound"),
                Transform::from_xyz(18.0, -12.0, 10.0),
                ConveyorHole::Outbound,
            ),
        ]),
    )
}

pub fn constructor_preview() -> impl Bundle {
    (
        Name::new("Constructor"),
        Sprite::from_color(Color::WHITE.with_alpha(0.5), Vec2::splat(64.0)),
        AnimatedMachine("constructor.aseprite"),
    )
}

pub fn power_pole() -> impl Bundle {
    (
        Name::new("Power Pole"),
        Sprite::sized(Vec2::splat(64.0)),
        AnimatedMachine("power-pole.aseprite"),
        PowerSockets::multiple(4),
        Interactable,
    )
}

pub fn power_pole_preview() -> impl Bundle {
    (
        Name::new("Power Pole Preview"),
        Sprite::from_color(Color::WHITE.with_alpha(0.5), Vec2::splat(64.0)),
        AnimatedMachine("power-pole.aseprite"),
    )
}
