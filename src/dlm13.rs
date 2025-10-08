// Custom Modules for Luajit
// IDK what the name means ...

use serde::Deserialize;
use std::fs;
use std::path::Path;

use crate::VERSION;

#[derive(Debug, Deserialize)]
struct Config {
    name: String,
    version: String,
    description: String,
    license: String,
    entrypoint: String,
    luajitversion: luajitversion
}

#[derive(Debug, Deserialize)]
struct luajitversion {
    min: String,
    max: String
}

// Function to start running a Module
pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    println!("Not Implemented yet!");
    return Ok(());

    // TODO
    // Look for Index File
    // Check Version of the Index File
        // Version in 3 zahlen teilen
        // diese dann zahl für zahl vergleichen
    // Check if entry point file exists

    // Module Index File
    let index_file = "dlm13.yml";

    if !Path::new(index_file).exists() {
        eprintln!("Error: File '{}' not found!", index_file);
        return Ok(());
    }

    let yaml_str = fs::read_to_string(index_file)?;
    let config: Config = serde_yaml::from_str(&yaml_str)?;

    Ok(())
}
