use bevy::{
    prelude::*,
    window::{close_on_esc, PresentMode, WindowMode},
};

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component)]
struct SnakeHead;

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake".to_string(),
            width: 1920.,
            height: 1080.,
            present_mode: PresentMode::AutoVsync,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_plugins(DefaultPlugins)
        .add_system(close_on_esc)
        .run();
}
