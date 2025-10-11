use bevy::prelude::*;
use crate::{cell, systems::set_grid};
use bevy::core_pipeline::core_2d::Camera2dBundle;

#[derive(Component)]
pub struct SetWindowButton;
#[derive(Event)]
pub struct ResetGridEvent;

pub fn set_window(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 🎮 Caméra pour la grille (z = 0)
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 0, // d'abord la grille
            ..default()
        },
        ..default()
    });

    // 🖱️ Caméra pour l’interface (z = 100)
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1, // puis l’UI par-dessus
            ..default()
        },
        ..default()
    });

    // Conteneur global (UI)
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Panneau de gauche
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(25.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.1, 0.1, 0.1).into(),
                    ..default()
                })
                .with_children(|ui| {
                    ui.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(160.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            background_color: Color::srgb(0.3, 0.6, 0.3).into(),
                            ..default()
                        },
                        SetWindowButton,
                    ))
                    .with_children(|b| {
                        b.spawn(TextBundle::from_section(
                            "Nouvelle Grille",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 22.0,
                                color: Color::WHITE,
                            },
                        ));
                    });
                });

            // Zone vide à droite (juste pour garder la structure flex)
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(75.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            });
        });
}


pub fn button_system(
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
pub fn reset_grid_system(
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