use crate::{
    cell,
    systems::{ StepTimer,MAX_SPEED, MIN_SPEED, set_grid},
    visual_elements::panel::{spawn_left_panel,spawn_right_panel},
    mutation_setting::{MutationCheckbox,MutationSettings}
    
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

pub const BAR_WIDTH: f32 = 100.0;
pub const HANDLE_WIDTH: f32 = 2.0;


pub fn set_window(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_cameras(&mut commands);
    spawn_ui_root(&mut commands, &asset_server);
}

fn spawn_cameras(commands: &mut Commands) {
    // 🎮 Caméra pour la grille (z = 0)
    commands.spawn(Camera2dBundle {
        camera: Camera { order: 0, ..default() },
        ..default()
    });

    // 🖱️ Caméra pour l’interface (z = 100)
    commands.spawn(Camera2dBundle {
        camera: Camera { order: 1, ..default() },
        ..default()
    });
}

fn spawn_ui_root(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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
            spawn_left_panel(parent, asset_server);
            spawn_right_panel(parent);
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


pub fn mutation_checkbox_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &MutationCheckbox), (Changed<Interaction>, With<Button>)>,
    mut settings: ResMut<MutationSettings>,
) {
    for (interaction, mut color, checkbox) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match checkbox {
                MutationCheckbox::Blue => {
                    settings.allow_blue = !settings.allow_blue;
                    *color = if settings.allow_blue {
                        Color::srgb(0.2, 0.2, 0.8).into()
                    } else {
                        Color::srgb(0.3, 0.3, 0.3).into()
                    };
                }
                MutationCheckbox::Red => {
                    settings.allow_red = !settings.allow_red;
                    *color = if settings.allow_red {
                        Color::srgb(0.8, 0.2, 0.2).into()
                    } else {
                        Color::srgb(0.3, 0.3, 0.3).into()
                    };
                }
            }
        }
    }
}