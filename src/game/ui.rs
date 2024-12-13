use bevy::{prelude::*, state::commands};

use super::{
    *
};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (show_title_and_name, show_level_title, show_dark_mode_button),
        );
        app.add_systems(Update, (
            dark_mode_button_interaction,
        ));
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
            bottom: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
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
            bottom: Val::Px(50.0),
            right: Val::Px(10.0),
            ..default()
        },
    ));
}

#[derive(Component)]
struct LevelTitle;

fn show_level_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new(format!("Level{}", 11)),
        TextFont {
            font: asset_server.load("fonts/NotJamChunky8.ttf"),
            font_size: 30.0,
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        LevelTitle,
    ));
}

fn show_dark_mode_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Px(100.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(5.)),
                position_type: PositionType::Absolute,
                bottom: Val::Px(100.0),
                right: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(MY_ORANGE),
            BorderColor(DARK_MODE_BG_COLOR),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Durk Mode"),
                TextFont {
                    font: asset_server.load("fonts/NotJamChunky8.ttf"),
                    font_size: 14.0,
                    ..Default::default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}


const NORMAL_BUTTON: Color = MY_ORANGE;
const HOVERED_BUTTON: Color =
    Color::rgb(222.0 / 255.0 + 0.1, 112.0 / 255.0 + 0.1, 40.0 / 255.0 + 0.1);
const PRESSED_BUTTON: Color = Color::rgb(0.75, 0.75, 0.75);

fn dark_mode_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for(interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::WHITE;
                // level_index.0 += 1;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = MY_BROWN;
            }
        }
    }
}
