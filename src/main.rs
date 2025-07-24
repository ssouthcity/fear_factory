use bevy::prelude::*;
use bevy_factory::FactoryGamePlugin;

fn main() -> AppExit {
    App::new().add_plugins(FactoryGamePlugin).run()
}
