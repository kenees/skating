use bevy::{ecs::query, prelude::*};

use crate::game;
pub struct GameInitPlugin;

impl Plugin for GameInitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(game::LIGHT_MODE_BG_COLOR));
        app.add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(
                    Window {
                        title: "My bevy test".to_string(),
                        mode: bevy::window::WindowMode::Windowed,
                        ..default()
                    }
                ),
                ..default()
            }
        ));
        app.add_plugins(game::GamePlugin);
        
        // app.add_systems(Startup, add_poeple);
        // app.add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}