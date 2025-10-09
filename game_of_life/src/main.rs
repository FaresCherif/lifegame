mod cell;
mod systems;

use bevy::{core_pipeline::core_2d::Camera2dBundle, prelude::*};
use systems::update_cells;
use systems::set_grid;

#[derive(Component)]
struct SetWindowButton;
#[derive(Event)]
struct ResetGridEvent;

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

fn set_window(mut commands: Commands, asset_server: Res<AssetServer>){
    // Crée une caméra 2D
    commands.spawn(Camera2dBundle::default());

    
        // Conteneur principal (centré)
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column, // colonne verticale
                justify_content: JustifyContent::FlexStart, // centré horizontalement
                align_items: AlignItems::FlexStart,      // en haut verticalement
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(160.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.3, 0.6, 0.3).into(),
                        ..default()
                    },
                    SetWindowButton,
                ))
                .with_children(|b| {
                    b.spawn(TextBundle::from_section(
                        "Set Window",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 22.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });


        
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SetWindowButton>)
    >,
    mut ev_reset: EventWriter<ResetGridEvent>, // 👈 on peut envoyer un event
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.4, 0.8, 0.4).into();
                ev_reset.send(ResetGridEvent); // 👈 déclenche le reset

            }
            Interaction::Hovered => {
                *color = Color::srgb(0.35, 0.7, 0.35).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.3, 0.6, 0.3).into();
            }
        }
    }
}


// 👇 Ce système écoute l’événement et appelle ton vrai système set_grid
fn reset_grid_system(
    mut ev_reset: EventReader<ResetGridEvent>,
    mut commands: Commands,
    query: Query<Entity, With<cell::Cell>>,
) {
    for _ in ev_reset.read() {
        // Supprimer toutes les anciennes cellules
        for e in query.iter() {
            commands.entity(e).despawn();
        }

        // Recréer une nouvelle grille
        set_grid(&mut commands);
    }
}