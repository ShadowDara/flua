// Rust to open Folderselecting and File / Files Selecting Dialogue

use std::error::Error;

use rfd::FileDialog;

// Function to select a Folder via a Folderselecting Dialogue
pub fn select_folder() -> Result<String, Box<dyn Error>> {
    if let Some(folder_path) = FileDialog::new().pick_folder() {
        println!("Selected folder: {:?}", folder_path);
        Ok(folder_path.display().to_string())
    } else {
        Err("No folder selected".into())
    }
}

// Function to select one File via a Fileselecting Dialogue
pub fn select_file() -> Result<String, Box<dyn Error>> {
    if let Some(file) = FileDialog::new().pick_file() {
        Ok(file.display().to_string())
    } else {
        Err("No file selected".into())
    }
}

// Function to select multiple files via file selecting Dialogue
pub fn select_files() -> Result<Vec<String>, Box<dyn Error>> {
    if let Some(paths) = FileDialog::new().pick_files() {
        let file_paths: Vec<String> = paths.iter().map(|p| p.display().to_string()).collect();

        println!("Selected files: {:?}", file_paths);
        Ok(file_paths)
    } else {
        Err("No files selected".into())
    }
}
