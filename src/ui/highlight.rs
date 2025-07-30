use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Highlightable>();

    app.add_systems(
        Update,
        (
            highlight.run_if(on_event::<Pointer<Over>>),
            unhighlight.run_if(on_event::<Pointer<Out>>),
        ),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Highlightable;

fn highlight(
    mut events: EventReader<Pointer<Over>>,
    mut sprites: Query<&mut Sprite, With<Highlightable>>,
) {
    for event in events.read() {
        if let Ok(mut sprite) = sprites.get_mut(event.target) {
            sprite.color = Color::hsl(60.0, 1.0, 0.5);
        }
    }
}

fn unhighlight(
    mut events: EventReader<Pointer<Out>>,
    mut sprites: Query<&mut Sprite, With<Highlightable>>,
) {
    for event in events.read() {
        if let Ok(mut sprite) = sprites.get_mut(event.target) {
            sprite.color = Default::default();
        }
    }
}
