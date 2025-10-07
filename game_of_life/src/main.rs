use bevy::{
    prelude::*,
    core_pipeline::core_2d::Camera2dBundle
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Crée une caméra 2D
    commands.spawn(Camera2dBundle::default());
}