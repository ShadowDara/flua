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
    author: String,
    license: String,
    link: String,
    entrypoint: String,
    luajitversion: Luajitversion,
    edition: i16,
}

#[derive(Debug, Deserialize)]
struct Luajitversion {
    min: String,
    max: String,
}

// Function to start running a Module
pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    println!("Not Implemented yet!");
    return Ok(());

    // Module Index File
    let index_file = "dlm13.yml";

    if !Path::new(index_file).exists() {
        eprintln!("Error: File '{}' not found!", index_file);
        return Ok(());
    }

    let yaml_str = fs::read_to_string(index_file)?;
    let config: Config = serde_yaml::from_str(&yaml_str)?;

    // Check if the edition is correct
    match config.edition {
        2025 => {}
        _ => {}
    };

    // Stop when the version is not fitting
    if !(is_newer_version(VERSION, config.luajitversion.min)
        && is_newer_version(VERSION, config.luajitversion.max))
    {
        return Err("Error".into());
    }

    let entrypoint: &'static str = &config.entrypoint;

    // Check if Index File exists
    if !Path::new(entrypoint).exists() {
        eprintln!("Error: Entrypoint '{}' not found!", entrypoint);
        return Ok(());
    }

    Ok(())
}

// Function to check if the current version is newer than another
fn is_newer_version(current_version: &str, the_version: String) -> bool {
    let v1 = parse_version(&current_version);
    let v2 = parse_version(&the_version);

    if v1[0] > v2[0] {
        return true;
    }
    if v1[1] > v2[1] {
        return true;
    }
    if v1[2] > v2[2] {
        return true;
    }

    return false;
}

// Numbers are not allowed to above 255!
fn parse_version(input: &str) -> [u8; 3] {
    let parts: Vec<u8> = input
        .split(".")
        .take(3)
        .map(|s| s.parse::<u8>())
        .collect::<Result<_, _>>()
        .expect("Version syntax is wrong, it does fit in u8 (numbers from 0 to 255)");

    match parts.as_slice() {
        [a, b, c] => [*a, *b, *c],
        _ => panic!("Version must contain 3 numbers!"),
    }
}
