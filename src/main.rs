#![windows_subsystem = "windows"]

use bevy::{prelude::*};

use bevy::core_pipeline::clear_color::ClearColorConfig;

mod states;

use states::StatesPlugin;

#[derive(Component)]
struct Camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(StatesPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>
) {
    for mut window in windows.iter_mut() {
        window.set_title(String::from("CyberHarvest: Data Breach"));
        window.set_resizable(false);
    }

    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.0)),
        },
        ..Default::default()
    });
}