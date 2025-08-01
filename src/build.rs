use bevy::prelude::*;

use crate::{
    FactorySystems,
    machine::prefabs::{CoalGenerator, Constructor, Miner, Windmill},
    power::pole::PowerPole,
    sandbox::Sandbox,
};

pub fn plugin(app: &mut App) {
    app.register_type::<Building>();
    app.register_type::<Buildable>();
    app.register_type::<QueueSpawnBuilding>();

    app.add_event::<QueueSpawnBuilding>();

    app.add_systems(
        Update,
        spawn_buildings
            .run_if(on_event::<QueueSpawnBuilding>)
            .in_set(FactorySystems::Build),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Building(Buildable);

#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Reflect, Debug, Clone, Copy)]
pub enum Buildable {
    #[default]
    Windmill,
    PowerPole,
    Miner,
    CoalGenerator,
    Constructor,
}

#[derive(Event, Reflect)]
pub struct QueueSpawnBuilding {
    pub buildable: Buildable,
    pub position: Vec2,
}

fn spawn_buildings(
    mut events: EventReader<QueueSpawnBuilding>,
    mut commands: Commands,
    world: Single<Entity, With<Sandbox>>,
) {
    for event in events.read() {
        let mut entity = commands.spawn((
            Transform::from_translation(event.position.extend(1.0)),
            ChildOf(*world),
            Building(event.buildable),
        ));

        match event.buildable {
            Buildable::Windmill => entity.insert(Windmill),
            Buildable::PowerPole => entity.insert(PowerPole),
            Buildable::Miner => entity.insert(Miner),
            Buildable::CoalGenerator => entity.insert(CoalGenerator),
            Buildable::Constructor => entity.insert(Constructor),
        };
    }
}
