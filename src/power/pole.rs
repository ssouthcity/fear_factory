use bevy::prelude::*;

use crate::{animation::AnimatedMachine, power::socket::PowerSockets};

pub fn plugin(app: &mut App) {
    app.register_type::<PowerPole>();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Name::new("Power Pole"),
    AnimatedMachine("power-pole.aseprite"),
    Sprite::sized(Vec2::splat(64.0)),
    PowerSockets::multiple(3)
)]
pub struct PowerPole;
