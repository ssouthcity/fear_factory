use bevy::prelude::*;

mod gameplay;
mod loading;

pub fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((loading::plugin, gameplay::plugin));
}

#[derive(States, Hash, Debug, PartialEq, Eq, Clone, Copy, Default)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Loading,
    Gameplay,
}
