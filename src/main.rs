use bevy::prelude::*;

mod game;
mod plugins;

fn main() {
    App::new()
    .add_plugins(plugins::game_init::GameInit)
    .run();
}