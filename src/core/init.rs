use crate::core::types::{KumulusConfig, KumulusInit};
use std::{fs, io, path::Path};

pub fn init(args: KumulusInit) {
    let config = KumulusConfig {
        project_name: args.project_name,
        output_directory: args.output_directory,
    };

    let config_path = "kumulus.json";

    if Path::new(config_path).exists() {
        println!("A `kumulus.json` file already exists. Overwrite? (y/N)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            println!("Aborting initialization.");
            return;
        }
    }

    let json = serde_json::to_string_pretty(&config).expect("Failed to serialize config");
    fs::write(config_path, json).expect("Failed to write config file");

    println!("✅ Project initialized successfully!");
}
