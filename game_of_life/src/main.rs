mod cell;
mod systems;
mod interface;
mod visual_elements;
mod mutation_setting;

use bevy::{prelude::*};
use systems::{set_grid,update_cells,StepTimer,DEFAULT_SPEED};
use interface::{set_window,update_slider, button_system, reset_grid_system,mutation_checkbox_system, ResetGridEvent};
use mutation_setting::{MutationSettings};

fn main() {
    App::new()
        .insert_resource(StepTimer{
            timer : Timer::from_seconds(DEFAULT_SPEED, TimerMode::Repeating),
            speed: DEFAULT_SPEED,
        })
        .insert_resource(MutationSettings::default())
        .add_plugins(DefaultPlugins)
        .add_event::<ResetGridEvent>() // <- très important
        .add_systems(Startup, (set_window,setup).chain())
        .add_systems(Update, (update_slider,update_cells,button_system,reset_grid_system,mutation_checkbox_system).chain())
        .run();
}

fn setup(mut commands: Commands) {
    set_grid(&mut commands); // Appel de ta fonction utilitaire
}




