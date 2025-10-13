mod cell;
mod systems;
mod interface;
mod visual_elements;

use bevy::{prelude::*};
use systems::{set_grid,update_cells,StepTimer,DEFAULT_SPEED};
use interface::{set_window,update_slider, button_system, reset_grid_system, ResetGridEvent};



fn main() {
    App::new()
        .insert_resource(StepTimer{
            timer : Timer::from_seconds(DEFAULT_SPEED, TimerMode::Repeating),
            speed: DEFAULT_SPEED,
        })
        .add_plugins(DefaultPlugins)
        .add_event::<ResetGridEvent>() // <- très important
        .add_systems(Startup, (set_window,setup).chain())
        .add_systems(Update, (update_slider,update_cells,button_system,reset_grid_system).chain())
        .run();
}

fn setup(mut commands: Commands) {
    set_grid(&mut commands); // Appel de ta fonction utilitaire
}




