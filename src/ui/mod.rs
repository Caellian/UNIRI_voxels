use bevy::prelude::*;

pub mod debug;
pub mod main_menu;

pub trait Menu {
    fn setup(commands: Commands, asset_server: Res<AssetServer>);
}
