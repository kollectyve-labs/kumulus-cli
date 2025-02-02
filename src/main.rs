
use clap::Parser;
mod core;
use  core::{ types::{Args, Commands}, auth::{login, logout}, resources::list_resources};


fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_resources();
        },
        Some(Commands::Login)=> {
            login();
        },
        Some(Commands::Logout)=> {
            logout();
        },
        None => {
            println!("Run with --help to see the instructions");
        }
    }
}
