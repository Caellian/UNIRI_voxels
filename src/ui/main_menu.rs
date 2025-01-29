use bevy::prelude::*;

use super::Menu;

pub struct MainMenu;

impl Menu for MainMenu {
    fn setup(commands: Commands, asset_server: Res<AssetServer>) {}
}
