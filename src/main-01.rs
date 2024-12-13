use bevy::{ecs::query, prelude::*};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

// ============================系统相关==========================================
// 启动系统，类似普通系统，但只运行一次
fn add_poeple(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
    println!("add poeple");
}

// 普通系统
// 范围：参数定义了运行范围 ，在有Name，Person组件的实体上运行
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break;
        }
    }
}

// 普通系统
fn hello_world() {
    // println!("Hello, world!!!");
}

//========================插件相关=======================
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        // 只执行一次
        app.add_systems(Startup, add_poeple);
        // loop ，chain 让系统按照代码顺序运行
        app.add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}

fn main() {
    App::new()
        // 添加插件，默认插件包含2d/3d渲染器，资产加载，ui系统，窗口和输入
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
