use bevy::{prelude::*, state::commands, window::PrimaryWindow};

pub const DARK_MODE_BG_COLOR: Color = Color::srgb(45.0 / 255.0, 47.0 / 255.0, 47.0 / 255.0);



pub struct GamePlugin;

impl bevy::app::Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // app.init_state()
        // app.add_plugins(plugins);
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {

}