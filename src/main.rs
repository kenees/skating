
use bevy::{asset::AssetMetaCheck, prelude::*};

mod game;
mod game_plugin;

fn main() {
    App::new()
    .add_plugins(game_plugin::game_init::GameInitPlugin)
    .run();
}