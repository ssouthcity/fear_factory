use bevy::prelude::*;

use crate::{
    FactorySystems,
    animation::AnimatedMachine,
    logistics::{ItemCollection, ResourceOutput},
    machine::prefabs::{CoalGenerator, Constructor, Miner, Windmill},
    power::pole::PowerPole,
    sandbox::{Deposit, Sandbox, SandboxSpawnSystems},
    ui::{HotbarItemDeselected, HotbarItemSelected, YSort},
};

pub fn plugin(app: &mut App) {
    app.register_type::<Preview>();
    app.register_type::<Building>();
    app.register_type::<Buildable>();
    app.register_type::<QueueSpawnBuilding>();

    app.add_event::<QueueSpawnBuilding>();

    app.add_systems(
        Startup,
        spawn_preview.in_set(SandboxSpawnSystems::SpawnDeposits),
    )
    .add_observer(on_hotbar_selection)
    .add_observer(on_hotbar_deselection);

    app.add_systems(
        Update,
        spawn_buildings
            .run_if(on_event::<QueueSpawnBuilding>)
            .in_set(FactorySystems::Build),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Preview;

fn spawn_preview(mut commands: Commands, sandbox: Single<Entity, With<Sandbox>>) {
    commands.spawn((
        Preview::default(),
        Sprite::from_color(Color::WHITE.with_alpha(0.25), Vec2::splat(64.0)),
        AnimatedMachine("windmill.aseprite"),
        Visibility::Hidden,
        ChildOf(*sandbox),
        YSort::default(),
    ));
}

fn on_hotbar_selection(
    trigger: Trigger<HotbarItemSelected>,
    preview: Single<Entity, With<Preview>>,
    mut commands: Commands,
) {
    commands.entity(*preview).insert((
        Visibility::Inherited,
        AnimatedMachine(match trigger.0 {
            Buildable::Windmill => "windmill.aseprite",
            Buildable::PowerPole => "power-pole.aseprite",
            Buildable::Miner => "miner.aseprite",
            Buildable::CoalGenerator => "coal-generator.aseprite",
            Buildable::Constructor => "constructor.aseprite",
        }),
    ));
}

fn on_hotbar_deselection(
    _trigger: Trigger<HotbarItemDeselected>,
    mut preview: Single<&mut Visibility, With<Preview>>,
) {
    **preview = Visibility::Hidden;
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(YSort::default())]
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
    pub placed_on: Entity,
}

fn spawn_buildings(
    mut events: EventReader<QueueSpawnBuilding>,
    mut commands: Commands,
    world: Single<Entity, With<Sandbox>>,
    deposits: Query<&Deposit>,
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
            Buildable::Miner => {
                if let Ok(deposit) = deposits.get(event.placed_on) {
                    entity.insert(ResourceOutput(
                        ItemCollection::new().with_item(deposit.0, 5),
                    ));
                }
                entity.insert(Miner)
            }
            Buildable::CoalGenerator => entity.insert(CoalGenerator),
            Buildable::Constructor => entity.insert(Constructor),
        };
    }
}
