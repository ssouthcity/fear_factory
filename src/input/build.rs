use bevy::prelude::*;

use crate::{
    input::{CursorPosition, InputMode},
    machine::prefabs::{BuildingType, CoalGenerator, Constructor, Miner, Windmill},
    power::pole::PowerPole,
    ui::HotbarSelection,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(InputMode::Build), spawn_observers);
}

fn spawn_observers(mut commands: Commands) {
    commands.spawn((Observer::new(spawn_building), StateScoped(InputMode::Build)));
}

fn spawn_building(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    cursor_position: Res<CursorPosition>,
    selected_buildable: Res<HotbarSelection>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }

    let mut entity = commands.spawn(Transform::from_translation(cursor_position.0.extend(1.0)));

    match selected_buildable.0 {
        BuildingType::Windmill => entity.insert(Windmill),
        BuildingType::PowerPole => entity.insert(PowerPole),
        BuildingType::Miner => entity.insert(Miner),
        BuildingType::CoalGenerator => entity.insert(CoalGenerator),
        BuildingType::Constructor => entity.insert(Constructor),
    };
}
