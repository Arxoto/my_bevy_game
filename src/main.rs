#![allow(dead_code)]

mod examples;
mod example_games;
mod my_games;

use bevy::prelude::*;

// =========
// course_1 hello Bevy ECS
// =========

/// ECS 设计模式  松耦合 数据驱动（分支预测，高性能）
///
/// Entity    实体 具有实际意义的实例化对象 在本设计模式中 仅是一个标识符 通过关联各个组件来拥有内部属性
///
/// Component 组件 实体具有的不同状态数据等 组件之间相互独立
///
/// System    系统 处理组件的逻辑
fn course_1() {
    // 这里会打开一个什么都没有的窗口（ DefaultPlugins 引入的窗口相关插件）
    // 且所有的 hello 都会无限循环（ DefaultPlugins 引入了事件循环）
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        // 一般来说各个 system 之间是并行的 如这里的 hello_world  如有需要使用 chain 来保证运行顺序
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don’t need to change any other names
        }
    }
}

fn hello_world() {
    println!("hello world!");
}

// =========
// course_2 Plugin and Resource
// =========

/// Plugin    插件 Bevy 中所有模块都以插件的形式实现 如 UI 渲染 等等 也可以自定义实现插件
///
/// Resource  资源 即全局唯一的数据（实体组件一般是大量可重复的数据）
fn course_2() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // 两秒循环的计时器
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people_with_time_res);
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people_with_time_res(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn main() {
    // course_1();
    // course_2();

    // examples::main();

    // example_games::main();

    // my_games::main();

    print!("{}\n", interpreter::add(2, 3));
}
