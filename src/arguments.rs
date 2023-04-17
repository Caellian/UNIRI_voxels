use bevy::prelude::Resource;
use clap::Parser;

#[derive(Parser, Resource, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Context {
    #[arg(long)]
    debug_asset_loader: bool,
}
