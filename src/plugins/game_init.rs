use bevy::{prelude::*};

use crate::game;
pub struct GameInit;

impl bevy::app::Plugin for GameInit {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(game::DARK_MODE_BG_COLOR)); // 设置clear颜色
        app.add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(
                        Window {
                            title: "my test".into(),
                            mode: bevy::window::WindowMode::Windowed,
                            prevent_default_event_handling: true,
                            ..Default::default()
                        }
                    ),
                    ..Default::default()
                }
            )
            // .set(ImagePlugin::default_nearest())
        );
    }
}