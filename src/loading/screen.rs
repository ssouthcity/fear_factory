use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);

    app.add_observer(on_set_loading_text);
}

#[derive(Event)]
pub(super) struct SetLoadingText(pub String);

#[derive(Component)]
struct LoadingText;

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        Name::new("Container"),
        DespawnOnExit(Screen::Loading),
        Node {
            width: percent(100.0),
            height: percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        children![(
            LoadingText,
            Name::new("Loading Text"),
            Text::new("Loading..."),
        )],
    ));
}

fn on_set_loading_text(
    set_loading_text: On<SetLoadingText>,
    mut loading_text: Single<&mut Text, With<LoadingText>>,
) {
    loading_text.0 = set_loading_text.0.clone();
}
