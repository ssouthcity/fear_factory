use bevy::prelude::*;

use crate::{
    input::InputMode,
    machine::prefabs::{BuildingType, CoalGenerator, Constructor, Miner, Windmill},
    power::pole::PowerPole,
    ui::HotbarSelection,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_plane);
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct World;

fn spawn_plane(mut commands: Commands) {
    commands
        .spawn((
            World::default(),
            Sprite::from_color(Color::NONE, Vec2::splat(2000.0)),
            Pickable::default(),
        ))
        .observe(spawn_building);
}

fn spawn_building(
    trigger: Trigger<Pointer<Click>>,
    input_mode: Res<State<InputMode>>,
    mut commands: Commands,
    selected_buildable: Res<HotbarSelection>,
) {
    if *input_mode.get() != InputMode::Build {
        return;
    }

    if trigger.event().button != PointerButton::Primary {
        return;
    }

    let Some(cursor_position) = trigger.event().hit.position else {
        return;
    };

    let mut entity = commands.spawn((
        Transform::from_translation(cursor_position.with_z(1.0)),
        ChildOf(trigger.target()),
    ));

    match selected_buildable.0 {
        BuildingType::Windmill => entity.insert(Windmill),
        BuildingType::PowerPole => entity.insert(PowerPole),
        BuildingType::Miner => entity.insert(Miner),
        BuildingType::CoalGenerator => entity.insert(CoalGenerator),
        BuildingType::Constructor => entity.insert(Constructor),
    };
}
