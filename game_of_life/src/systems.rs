use bevy::prelude::*;
use crate::{
    cell::{
        Cell
    },
    mutation_setting::MutationSettings
}; // pour accéder au composant
use std::collections::HashMap;

#[derive(Resource)]
pub struct StepTimer{
    pub timer: Timer,
    pub speed: f32,
}

const GRID_WIDTH: usize = 20;
const GRID_HEIGHT: usize = 20;
const CELL_SIZE: f32 = 20.0;
pub const DEFAULT_SPEED: f32 = 0.05;
pub const MIN_SPEED: f32 = 1.0;
pub const MAX_SPEED: f32 = 0.05;

pub fn update_cells(
    time: Res<Time>,
    mut timer: ResMut<StepTimer>,
    mut query: Query<(&mut Cell, &mut Sprite)>,
    settings: Res<MutationSettings>
) {
    // 🔹 On fait avancer le timer à chaque frame
    if !timer.timer.tick(time.delta()).just_finished() {
        return; // ⛔ On ne fait rien tant que le timer n’a pas fini
    }

    // 1️⃣ Construire une map (x,y) -> alive
    let mut grid_map: HashMap<(usize, usize), bool> = HashMap::new();
    for (cell, _sprite) in query.iter() {
        grid_map.insert((cell.x, cell.y), cell.alive);
    }

// 2️⃣ Calculer le prochain état pour chaque cellule
    let mut updates: Vec<((usize, usize), bool)> = Vec::new();
    for (cell, _sprite) in query.iter() {
        let mut alive_neighbors = 0;

        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = cell.x as i32 + dx;
                let ny = cell.y as i32 + dy;
                if nx >= 0 && nx < GRID_WIDTH as i32 && ny >= 0 && ny < GRID_HEIGHT as i32 {
                    if *grid_map.get(&(nx as usize, ny as usize)).unwrap_or(&false) {
                        alive_neighbors += 1;
                    }
                }
            }
        }

        // Appliquer les règles classiques

        updates.push(((cell.x, cell.y), cell.change_state(alive_neighbors)));
    }

    // 3️⃣ Appliquer les changements aux cellules et aux sprites
    for ((x, y), next_state) in updates {
        for (mut cell, mut sprite) in query.iter_mut() {
            if cell.x == x && cell.y == y {

                // mutation seulement si la cellule "renaît"
                if !cell.alive && next_state {
                    cell.random_mutation(&settings);
                }

                cell.alive = next_state;
                sprite.color = cell.cell_color(); // couleur cohérente
                break;
            }
        }
    }
}

pub fn set_grid(commands: &mut Commands,settings: &Res<MutationSettings>) {

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let cell = Cell::new(x,y,settings);


            let pos_x = (x as f32 - GRID_WIDTH as f32 / 2.0) * CELL_SIZE;
            let pos_y = (y as f32 - GRID_HEIGHT as f32 / 2.0) * CELL_SIZE;

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: cell.cell_color(),
                        custom_size: Some(Vec2::splat(CELL_SIZE - 1.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(pos_x, pos_y, 0.0),
                    ..Default::default()
                })
                .insert(cell);
        }
    }
}



