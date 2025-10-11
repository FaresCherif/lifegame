mod cell;
mod systems;
mod interface;

use bevy::{prelude::*};
use systems::update_cells;
use systems::set_grid;
use interface::{set_window, button_system, reset_grid_system, ResetGridEvent};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<ResetGridEvent>() // <- très important
        .add_systems(Startup, (set_window,setup).chain())
        .add_systems(Update, (update_cells,button_system,reset_grid_system).chain())
        .run();
}

fn setup(mut commands: Commands) {
    set_grid(&mut commands); // Appel de ta fonction utilitaire
}




