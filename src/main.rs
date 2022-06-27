// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::prelude::{App, ClearColor, Color, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("position: {:?}", transform.translation);
    }
}
struct Entity(u64);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Renzo Hume".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Zayna Nieves".to_string()));
}

fn hello_world() {
    println!("hello world!");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people)
            .add_system(hello_world)
            .add_system(greet_people);
    }
}

fn add_rect(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        ..default()
    });
}
pub struct RectPlugin;
impl Plugin for RectPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_rect);
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Bevy game".to_string(), // ToDo
            #[cfg(target_arch = "wasm32")]
            canvas: Some("canvas".to_string()),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_plugin(RectPlugin)
        .run();
}
