use bevy::prelude::*;

use crate::screens::Screen;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_gameplay);
}

fn spawn_gameplay(mut commands: Commands) {
    commands.spawn((
        Name::new("Container"),
        StateScoped(Screen::Gameplay),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(Name::new("Header"), Text::new("GAMING!"),)],
    ));
}
