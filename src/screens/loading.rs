use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::once_after_delay};

use crate::screens::Screen;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);

    app.add_systems(
        Update,
        transition_to_gameplay
            .run_if(once_after_delay(Duration::from_secs(1)))
            .run_if(in_state(Screen::Loading)),
    );
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        Name::new("Container"),
        StateScoped(Screen::Loading),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        children![(Name::new("Loading Progress"), Text::new("Loading..."),)],
    ));
}

fn transition_to_gameplay(mut next_state: ResMut<NextState<Screen>>) {
    next_state.set(Screen::Gameplay);
}
