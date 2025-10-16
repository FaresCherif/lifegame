use bevy::prelude::*;

#[derive(Resource)]
pub struct MutationSettings {
    pub allow_blue: bool,
    pub allow_red: bool,
}

#[derive(Component)]
pub enum MutationCheckbox {
    Blue,
    Red,
}

impl Default for MutationSettings {
    fn default() -> Self {
        Self {
            allow_blue: true,
            allow_red: true,
        }
    }
}
