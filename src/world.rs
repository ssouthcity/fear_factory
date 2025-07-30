use bevy::prelude::*;

use crate::{
    FactorySystems,
    machine::prefabs::{CoalGenerator, Constructor, Miner, Windmill},
    power::pole::PowerPole,
    ui::HotbarSelection,
};

pub fn plugin(app: &mut App) {
    app.add_event::<SpawnBuilding>();

    app.add_systems(Startup, spawn_world);

    app.add_systems(
        Update,
        spawn_buildings
            .run_if(on_event::<SpawnBuilding>)
            .in_set(FactorySystems::Build),
    );
}

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
struct SpawnBuilding {
    buildable: Buildable,
    position: Vec2,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct World;

fn spawn_world(mut commands: Commands) {
    commands
        .spawn((
            Name::new("World"),
            World::default(),
            Sprite::from_color(Color::hsl(100.0, 0.5, 0.64), Vec2::splat(1600.0)),
            Pickable::default(),
        ))
        .observe(queue_spawn_building);
}

fn queue_spawn_building(
    trigger: Trigger<Pointer<Click>>,
    mut events: EventWriter<SpawnBuilding>,
    selected_buildable: Res<HotbarSelection>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }

    let Some(cursor_position) = trigger.event().hit.position else {
        return;
    };

    events.write(SpawnBuilding {
        buildable: selected_buildable.0,
        position: cursor_position.truncate(),
    });
}

fn spawn_buildings(
    mut events: EventReader<SpawnBuilding>,
    mut commands: Commands,
    world: Single<Entity, With<World>>,
) {
    for event in events.read() {
        let mut entity = commands.spawn((
            Transform::from_translation(event.position.extend(1.0)),
            ChildOf(*world),
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
