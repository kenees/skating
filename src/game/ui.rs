use bevy::{ prelude::*, state::commands };

use super:: {
    level::CurrentLevelIndex,
    *,
};


pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, show_title_and_name);
    }
}

fn show_title_and_name(mut commands: Commands, asset_server: Res<AssetServer>) {
    // title
    commands.spawn((
        Text::new("QUACK!!! on ICE"),
        TextFont {
            font: asset_server.load("fonts/NotJamChunky8.ttf"),
            font_size: 30.0,
            ..Default::default()
        },
        TextColor(MY_ORANGE),
        TextLayout::new_with_justify(JustifyText::Right),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        }
    ));

    // auth name
    commands.spawn((
        Text::new("a game by Minda Chen"),
        TextFont {
            font: asset_server.load("fonts/NotJamChunky8.ttf"),
            font_size: 20.0,
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Right),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            right: Val::Px(10.0),
            ..default()
        }
    ));
}


#[derive(Component)]
struct LevelTitle;

fn show_level_title(mut commands: Commands, asset_server: Res<AssetServer>, level_index: Res<CurrentLevelIndex>) {
    commands.spawn(
        (
            Text::new(format!("Level{}", 11)),
            TextFont {
                font: asset_server.load("fonts/NotJamChunky8.ttf"),
                font_size: 30.0,
                ..Default::default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            },
            LevelTitle
        )
    );
}