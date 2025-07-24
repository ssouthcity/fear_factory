use bevy::prelude::*;

pub struct FactoryGamePlugin;

impl Plugin for FactoryGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);

        app.insert_resource(ClearColor(Color::linear_rgb(0.25, 0.4, 0.0)));

        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Sprite::from_color(Color::linear_rgb(0.0, 0.0, 1.0), Vec2::splat(1028.0)),
            Pickable::default(),
        ))
        .observe(spawn_building);
}

fn spawn_building(trigger: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.spawn((
        Transform::from_translation(trigger.event().hit.position.unwrap_or_default()),
        Sprite::from_color(Color::linear_rgb(1.0, 0.0, 0.0), Vec2::splat(64.0)),
    ));
}
