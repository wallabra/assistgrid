use bevy::prelude::*;

#[derive(Resource)]
pub struct World {}

impl Default for World {
    fn default() -> Self {
        World {}
    }
}
