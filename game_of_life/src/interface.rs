use crate::{
    cell,
    systems::StepTimer,
    systems::{DEFAULT_SPEED, MAX_SPEED, MIN_SPEED, set_grid},
};
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::ecs::system::ParamSet;
use bevy::prelude::*;

#[derive(Component)]
pub struct SetWindowButton;
#[derive(Event)]
pub struct ResetGridEvent;

#[derive(Component)]
pub struct SpeedSlider;

#[derive(Component)]
pub struct SliderHandle;

#[derive(Component)]
pub struct SpeedText;

const BAR_WIDTH: f32 = 100.0;
const HANDLE_WIDTH: f32 = 2.0;

pub fn set_window(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ratio = (MIN_SPEED - DEFAULT_SPEED) / (MIN_SPEED - MAX_SPEED);

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

                    // --- BARRE DE VITESSE + TEXTE ---
                    ui.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(30.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    })
                    .with_children(|parent| {

                        // --- BARRE ---
                        parent
                            .spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Px(BAR_WIDTH),
                                        height: Val::Px(20.0),
                                        position_type: PositionType::Relative,
                                        justify_content: JustifyContent::FlexStart,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    background_color: Color::srgb(0.3, 0.3, 0.3).into(), // ✅ Gris clair
                                    border_color: BorderColor(Color::WHITE), // ✅ Bordure blanche
                                    ..default()
                                },
                                SpeedSlider,
                            ))
                            .with_children(|bar| {
                                let handle_x = BAR_WIDTH - ( HANDLE_WIDTH * 2.0 + 2.0 );
                                bar.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Px(HANDLE_WIDTH),
                                            height: Val::Px(24.0),
                                            left: Val::Px(handle_x),
                                            position_type: PositionType::Absolute,
                                            ..default()
                                        },
                                        background_color: Color::srgb(1.0 - ratio, 1.0, 0.2).into(),
                                        ..default()
                                    },
                                    SliderHandle,
                                ));
                            });

                        // --- TEXTE À DROITE ---
                        parent.spawn((
                            TextBundle::from_section(
                                format!("{:.2} s/étape", DEFAULT_SPEED), // ✅ texte correct
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                },
                            ),
                            SpeedText,
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
        (Changed<Interaction>, With<SetWindowButton>),
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

pub fn update_slider(
    mut cursor: EventReader<CursorMoved>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut param_set: ParamSet<(
        Query<(&GlobalTransform, &Style), With<SpeedSlider>>,
        Query<(&mut Style, &mut BackgroundColor), With<SliderHandle>>,
    )>,
    mut text_query: Query<&mut Text, With<SpeedText>>,
    mut timer_res: ResMut<StepTimer>,
) {
    // 🧩 1️⃣ On lit d’abord les infos du slider
    let (slider_x, slider_width) = {
        let slider_query = param_set.p0();
        if let Ok((transform, style)) = slider_query.get_single() {
            let width = if let Val::Px(w) = style.width {
                w
            } else {
                BAR_WIDTH
            };
            (transform.translation().x, width)
        } else {
            return;
        }
    };

    // 🧩 2️⃣ Puis on modifie le handle (dans une autre portée)
    if let Ok((mut handle_style, mut handle_color)) = param_set.p1().get_single_mut() {
        for event in cursor.read() {
            if buttons.pressed(MouseButton::Left) {
                let x = event
                    .position
                    .x
                    .clamp(slider_x - slider_width / 2.0, slider_x + slider_width / 2.0);

                // ✅ vitesse
                let min_speed = MIN_SPEED;
                let max_speed = MAX_SPEED;

                let ratio = ((x - (slider_x - slider_width / 2.0)) / slider_width).clamp(0.0, 1.0);

                // ✅ déplacer le handle
                let mut pos = ratio * BAR_WIDTH - (HANDLE_WIDTH * 2.0);
                pos = pos.clamp(0.0, BAR_WIDTH - ((HANDLE_WIDTH * 2.0)+2.0));

                handle_style.left = Val::Percent(pos);

                let new_speed = min_speed - ratio * (min_speed - max_speed);

                timer_res.speed = new_speed;
                timer_res
                    .timer
                    .set_duration(std::time::Duration::from_secs_f32(new_speed));

                *handle_color = Color::srgb(1.0 - ratio, 1.0, 0.2).into();

                // ✅ texte
                if let Ok(mut text) = text_query.get_single_mut() {
                    text.sections[0].value = format!("{:.2} s/étape", new_speed);
                }
            }
        }
    }
}
