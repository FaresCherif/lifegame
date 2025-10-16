use bevy::prelude::*;
use crate::{
    visual_elements::controls::{spawn_new_grid_button,spawn_speed_control,spawn_mutation_checkboxes}
};

pub fn spawn_left_panel(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
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
            spawn_new_grid_button(ui, asset_server);
            spawn_speed_control(ui, asset_server);
            spawn_mutation_checkboxes(ui,asset_server)
        });
}

pub fn spawn_right_panel(parent: &mut ChildBuilder) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(75.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    });
}
