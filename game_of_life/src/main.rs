mod cell;
mod systems;

use bevy::{core_pipeline::core_2d::Camera2dBundle, prelude::*};
use systems::update_cells;
use systems::set_grid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (set_window,set_grid).chain())
        .add_systems(Update, update_cells)
        .run();
}

fn set_window(mut commands: Commands){
    // Crée une caméra 2D
    commands.spawn(Camera2dBundle::default());
}
