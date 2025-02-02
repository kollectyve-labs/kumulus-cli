use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Login via CLI
    Login,
    /// Logout via CLI
    Logout,
    /// List Ressources
    List,
}

#[derive(Debug)]
pub enum ResourceStatus {
    Running,
    Stopped,
}