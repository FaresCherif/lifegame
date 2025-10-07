use bevy::prelude::*;
use crate::cell::Cell; // pour accéder au composant

const GRID_WIDTH: usize = 20;
const GRID_HEIGHT: usize = 20;
const CELL_SIZE: f32 = 20.0;

pub fn update_cells(
    mut query: Query<(&mut Cell, &mut Sprite)>
) {
    for (mut cell, mut sprite) in query.iter_mut() {
        // toggle aléatoire
        if rand::random::<f32>() < 0.01 {
            cell.alive = !cell.alive;
            sprite.color = cell_color(cell.alive)
        }
    }
}

pub fn set_grid(mut commands: Commands) {

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let alive = rand::random::<bool>();
            let pos_x = (x as f32 - GRID_WIDTH as f32 / 2.0) * CELL_SIZE;
            let pos_y = (y as f32 - GRID_HEIGHT as f32 / 2.0) * CELL_SIZE;

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: if alive {
                            Color::srgb(0.0, 1.0, 0.0)
                        } else {
                            Color::srgb(0.0, 0.0, 0.0)
                        },
                        custom_size: Some(Vec2::splat(CELL_SIZE - 1.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(pos_x, pos_y, 0.0),
                    ..Default::default()
                })
                .insert(Cell { alive, x, y });
        }
    }
}



fn cell_color(alive: bool) -> Color {
    if alive {
        Color::srgb(0.2, 0.8, 0.2) // vert doux pour vivant
    } else {
        Color::srgb(0.0, 0.0, 0.0) // noir pour mort
    }
}