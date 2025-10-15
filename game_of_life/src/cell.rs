use bevy::prelude::*; // nécessaire ici car ce fichier a son propre scope
use rand::random;

// Composant représentant une cellule
#[derive(Component)]
pub struct Cell {
    pub alive: bool,       // état vivant ou mort
    pub mutation: MutationType,
    pub x: usize,          // position dans la grille
    pub y: usize,
}

/// Type de mutation possible
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MutationType {
    None,
    Blue,
    Red,
}



impl Cell {

    pub fn new(x: usize, y: usize) -> Self {
        let mut cell = Self {
            alive :rand::random::<bool>(),
            mutation: MutationType::None,
            x:x,
            y:y
        };
        cell.random_mutation(); // <--- on appelle la méthode ici
        cell
        
    }

    pub fn change_state(&self, alive_neighbors: usize) -> bool {
        return if self.alive {
            alive_neighbors == 2 || alive_neighbors == 3
        } else {
            alive_neighbors == 3
        };
    }


    /// Tire aléatoirement un type de mutation selon les probabilités
    pub fn random_mutation(&mut self) {
        
        let r = random::<f32>();
        self.mutation = if r < 0.1 {
            MutationType::Blue
        } else if r < 0.2 {
            MutationType::Red
        } else {
            MutationType::None
        }
    }
}