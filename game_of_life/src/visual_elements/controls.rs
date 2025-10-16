use bevy::prelude::*;
use crate::{
    interface::{
        SetWindowButton,SpeedSlider,SliderHandle,SpeedText,
        BAR_WIDTH,HANDLE_WIDTH
    },
    mutation_setting::{MutationCheckbox},
    systems::{MIN_SPEED,MAX_SPEED,DEFAULT_SPEED}
};


/************************************ New Grid ************************************/

//////////////
//  Button  //
////////////// 

pub fn spawn_new_grid_button(ui: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
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
}


/************************************ Speed ************************************/

///////////////
//  Control  //
/////////////// 

pub fn spawn_speed_control(ui: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    let ratio = (MIN_SPEED - DEFAULT_SPEED) / (MIN_SPEED - MAX_SPEED);

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
        spawn_speed_bar(parent, ratio);
        spawn_speed_text(parent, asset_server);
    });
}

/////////////
//// Bar ////
///////////// 

fn spawn_speed_bar(parent: &mut ChildBuilder, ratio: f32) {
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
                background_color: Color::srgb(0.3, 0.3, 0.3).into(),
                border_color: BorderColor(Color::WHITE),
                ..default()
            },
            SpeedSlider,
        ))
        .with_children(|bar| {
            let handle_x = BAR_WIDTH - (HANDLE_WIDTH * 2.0 + 2.0);
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
}

//////////////
//// Text ////
////////////// 

fn spawn_speed_text(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn((
        TextBundle::from_section(
            format!("{:.2} s/étape", DEFAULT_SPEED),
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 18.0,
                color: Color::WHITE,
            },
        ),
        SpeedText,
    ));
}

/************************************ Mutation ************************************/

//////////////
// CHECKBOX //
////////////// 


pub fn spawn_mutation_checkboxes(ui: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    ui.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        ..default()
    })
    .with_children(|col| {
        spawn_checkbox(col, font.clone(), "Mutation Bleue", MutationCheckbox::Blue,Color::srgb(0.2, 0.2, 0.8).into());
        spawn_checkbox(col, font.clone(), "Mutation Rouge", MutationCheckbox::Red,Color::srgb(0.8, 0.2, 0.2).into());
    });
}

fn spawn_checkbox(parent: &mut ChildBuilder, font: Handle<Font>, label: &str, kind: MutationCheckbox, bg_color: BackgroundColor) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|row| {
            // petit carré cliquable
            row.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(24.0),
                        height: Val::Px(24.0),
                        margin: UiRect::right(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: bg_color,
                    ..default()
                },
                kind,
            ));

            // texte à côté
            row.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font,
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });
}
