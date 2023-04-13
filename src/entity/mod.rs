pub mod player;

use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct Health(u16);
