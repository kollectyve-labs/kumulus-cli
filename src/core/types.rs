use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command()]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a Kumulus project
    Init(KumulusInit),
    /// Login via CLI
    Login,
    /// Logout via CLI
    Logout,
    /// List Resources
    List,
    /// Deploy a Web App
    Deploy,
}

/// CLI Arguments for `kumulus init`
#[derive(Clone, Parser)]
pub struct KumulusInit {
    /// Name of your project
    #[arg(long)]
    pub project_name: String,

    /// Directory containing static files (e.g., `dist` or `public`)
    #[arg(long, default_value = "dist")]
    pub output_directory: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KumulusConfig {
    pub project_name: String,
    pub output_directory: String,
}

#[derive(Debug)]
pub enum ResourceStatus {
    Running,
    Stopped,
}
