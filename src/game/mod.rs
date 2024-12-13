use bevy::{ prelude::*, render::camera::ScalingMode, state::commands, window::PrimaryWindow};
mod ui;
mod level;

pub const MY_ORANGE: Color = Color::srgb(222.0 / 255.0, 112.0 / 255.0, 40.0 / 255.0);
pub const DARK_MODE_BG_COLOR: Color = Color::srgb(45.0 / 255.0, 47.0 / 255.0, 47.0 / 255.0);

pub struct GamePlugin;

impl bevy::app::Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // 初始化游戏相关
        app.add_plugins(ui::Plugin); // ui控件
        app.add_plugins(level::Plugin);

        app.add_systems(Startup, spawn_camera); // 添加相机
    }
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap(); // ??
    let mut my_2d_camera_bundle = Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    };
    my_2d_camera_bundle.projection.scaling_mode = ScalingMode::FixedHorizontal { viewport_width: 1280.0 };
    commands.spawn(my_2d_camera_bundle);
}