use bevy::prelude::*;

use crate::{
    machine::prefabs::{BuildingType, CoalGenerator, Constructor, Miner, Windmill},
    power::pole::PowerPole,
    ui::HotbarSelection,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_world);
}

fn spawn_world(mut commands: Commands) {
    commands
        .spawn((
            Name::new("World"),
            Sprite::from_color(Color::hsl(100.0, 0.5, 0.64), Vec2::splat(1600.0)),
            Pickable::default(),
        ))
        .observe(spawn_building);
}

fn spawn_building(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    selected_buildable: Res<HotbarSelection>,
) {
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
