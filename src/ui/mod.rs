use bevy::prelude::*;

pub mod hud;
pub mod main_menu;

pub trait Menu {
    fn setup(commands: Commands, asset_server: Res<AssetServer>);
}
