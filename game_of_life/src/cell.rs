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

/// Tire aléatoirement un type de mutation selon les probabilités
pub fn random_mutation() -> MutationType {
    
    let r = random::<f32>();
    if r < 0.1 {
        MutationType::Blue
    } else if r < 0.2 {
        MutationType::Red
    } else {
        MutationType::None
    }
}