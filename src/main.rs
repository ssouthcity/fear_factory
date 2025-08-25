use bevy::prelude::*;
use fear_factory::FactoryGamePlugin;

fn main() -> AppExit {
    App::new().add_plugins(FactoryGamePlugin).run()
}
