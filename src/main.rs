use clap::Parser;
mod core;
use core::{
    auth::{login, logout},
    init::init,
    resources::list_resources,
    types::{Args, Commands},
};

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::Init(init_args)) => init(init_args),
        Some(Commands::List) => list_resources(),
        Some(Commands::Login) => login(),
        Some(Commands::Logout) => logout(),
        Some(Commands::Deploy) => println!("Deploying"),
        None => println!("Run with --help to see the instructions"),
    }
}
