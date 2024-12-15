use bevy::{prelude::*, state::commands};

use super::{*, level::{BreadCount, TotalBreadCount, CurrentLevelIndex}};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                show_title_and_name,
                show_hints,
                show_level_title,
                show_stuffed_ducks_count,
                show_dark_mode_button,
            ),
        )
        .add_event::<Won>()
        .add_systems(Update, 
            (
                won,
                update_level_title, 
                next_level_button_interaction,
                dark_mode_button_interaction,
            ),
        )
        .init_resource::<DisplayMode>();
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
struct StuffedDucksCount;

fn show_stuffed_ducks_count(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    bread_count: Res<BreadCount>,
    total_bread_count: Res<TotalBreadCount>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Text::new(format!("{}/{}", total_bread_count.0 -  bread_count.0, total_bread_count.0)),
        TextFont {
            font: asset_server.load("fonts/NotJamChunky8.ttf"),
            font_size: 30.0,
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(window.width() / 2.0 - 45.0),
            ..default()
        },
    ));
}

fn show_hints(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Text::default(),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(8.0),
                right: Val::Px(8.0),
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Right),
        ))
        .with_children(|p| {
            p.spawn((
                TextSpan("Click".to_string()),
                TextColor(MY_ORANGE),
                TextFont {
                    font: asset_server.load("fonts/NotJamChunky8.ttf"),
                    font_size: 20.0,
                    ..Default::default()
                },
            ));
            p.spawn((
                TextSpan::new(" to choose the duck \n"),
                TextFont {
                    font: asset_server.load("fonts/NotJamChunky8.ttf"),
                    font_size: 20.0,
                    ..Default::default()
                },
            ));
            p.spawn((
                TextSpan::new("WASD"),
                TextColor(MY_ORANGE),
                TextFont {
                    font: asset_server.load("fonts/NotJamChunky8.ttf"),
                    font_size: 20.0,
                    ..Default::default()
                }
            ));
            p.spawn((
                TextSpan::new(" to move \n"),
                TextFont {
                    font: asset_server.load("fonts/NotJamChunky8.ttf"),
                    font_size: 20.0,
                    ..Default::default()
                }
            ));
            p.spawn((
                TextSpan::new("R"),
                TextColor(MY_ORANGE),
                TextFont {
                    font: asset_server.load("fonts/NotJamChunky8.ttf"),
                    font_size: 20.0,
                    ..Default::default()
                }
            ));
            p.spawn((
                TextSpan::new(" to reset \n"),
                TextFont {
                    font: asset_server.load("fonts/NotJamChunky8.ttf"),
                    font_size: 20.0,
                    ..Default::default()
                }
            ));
            p.spawn((
                TextSpan::new("Z"),
                TextColor(MY_ORANGE),
                TextFont {
                    font: asset_server.load("fonts/NotJamChunky8.ttf"),
                    font_size: 20.0,
                    ..Default::default()
                }
            ));
            p.spawn((
                TextSpan::new(" to undo \n"),
                TextFont {
                    font: asset_server.load("fonts/NotJamChunky8.ttf"),
                    font_size: 20.0,
                    ..Default::default()
                }
            ));
            p.spawn((
                TextSpan::new("[ ]"),
                TextColor(MY_ORANGE),
                TextFont {
                    font: asset_server.load("fonts/NotJamChunky8.ttf"),
                    font_size: 20.0,
                    ..Default::default()
                }
            ));
            p.spawn((
                TextSpan::new(" to skip levels \n\n"),
                TextFont {
                    font: asset_server.load("fonts/NotJamChunky8.ttf"),
                    font_size: 20.0,
                    ..Default::default()
                }
            ));

            p.spawn((
                TextSpan::new("One duck, one bread\n"),
                TextFont {
                    font: asset_server.load("fonts/NotJamChunky8.ttf"),
                    font_size: 20.0,
                    ..Default::default()
                },
                TextColor(MY_ORANGE),
            ));

            p.spawn((
                TextSpan::new("Bigger duck, more bread"),
                TextFont {
                    font: asset_server.load("fonts/NotJamChunky8.ttf"),
                    font_size: 20.0,
                    ..Default::default()
                },
                TextColor(MY_ORANGE),
            ));
        });

}

#[derive(Component)]
struct LevelTitle;

fn show_level_title(mut commands: Commands, asset_server: Res<AssetServer>, level_index: Res<CurrentLevelIndex>) {
    commands.spawn((
        Text::new(format!("Level{}", level_index.0)),
        TextFont {
            font: asset_server.load("fonts/NotJamChunky8.ttf"),
            font_size: 30.0,
            ..Default::default()
        },
        TextColor(MY_ORANGE),
        // TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        LevelTitle,
    ));
}

fn update_level_title(
    mut commands: Commands, 
    level_index: Res<CurrentLevelIndex>,
    mut level_title: Query<&mut TextSpan, With<LevelTitle>>,
) {
    if level_index.is_changed() {
       for mut span in &mut level_title {
           **span = format!("Level{}", level_index.0);
       }
    }
}

const NORMAL_BUTTON: Color = MY_ORANGE;
const HOVERED_BUTTON: Color =
    Color::rgb(222.0 / 255.0 + 0.1, 112.0 / 255.0 + 0.1, 40.0 / 255.0 + 0.1);
const PRESSED_BUTTON: Color = Color::rgb(0.75, 0.75, 0.75);

#[derive(Event, Default)]
pub struct Won;

#[derive(Component)]
pub struct NextLevelButton;

#[derive(Component)]
struct DarkModeButton;

#[derive(Resource, Default)]
pub struct DisplayMode {
    dark_mode: bool,
}

#[derive(Component)]
pub struct MutUI;

fn show_dark_mode_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Button,
            DarkModeButton,
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

fn dark_mode_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>, With<DarkModeButton>),
    >,
    mut clear_color: ResMut<ClearColor>,
    mut display_mode: ResMut<DisplayMode>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::WHITE;
                println!("Pressed...");
                match display_mode.dark_mode {
                    false => clear_color.0 = DARK_MODE_BG_COLOR,
                    true => clear_color.0 = LIGHT_MODE_BG_COLOR,
                }
                display_mode.dark_mode = !display_mode.dark_mode;
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

fn won(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<Won>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    level_index: Res<CurrentLevelIndex>,
    levels: Res<level::Levels>,
) {
    for _ in events.read() {
        println!("???");
    }
}

fn next_level_button_interaction(
    mut interaction_query:Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>, With<NextLevelButton>),
    >,
    mut level_index: ResMut<CurrentLevelIndex>,
    levels: Res<level::Levels>,
) {
    for(interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                println!("button .....")
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}