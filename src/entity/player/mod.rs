pub mod fly_cam;

use crate::entity::Health;
use bevy::prelude::*;
use derive_more::{Deref, DerefMut};
use fly_cam::FlyCamera;

#[derive(Debug, Component)]
pub struct PlayerName(String);

impl Default for PlayerName {
    fn default() -> PlayerName {
        PlayerName("Player".to_string())
    }
}

#[derive(Debug, Default, Deref, DerefMut, Component)]
pub struct PlayerChunk(UVec3);

#[derive(Default, Bundle)]
pub struct Player {
    pub name: PlayerName,
    pub hp: Health,
    pub chunk: PlayerChunk,

    pub fly_cam: FlyCamera,
    #[bundle]
    pub camera: Camera3dBundle,
}

impl Player {
    pub fn new(name: impl AsRef<str>) -> Player {
        Player {
            name: PlayerName(name.as_ref().to_string()),
            hp: Health(100),
            ..Default::default()
        }
    }
}

// TODO: pub struct CameraMount;

// TODO: bundle players

pub fn spawn_player(mut commands: Commands) {
    commands.spawn(Player::new("Debug player"));
}
