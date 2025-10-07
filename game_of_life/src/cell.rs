use bevy::prelude::*; // nécessaire ici car ce fichier a son propre scope

// Composant représentant une cellule
#[derive(Component)]
pub struct Cell {
    pub alive: bool,       // état vivant ou mort
    pub x: usize,          // position dans la grille
    pub y: usize,
}