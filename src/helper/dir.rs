use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};

// Function to copy a dir recursivly
pub fn copy_dir_recursive(from: &Path, to: &Path) -> io::Result<()> {
    if !to.exists() {
        fs::create_dir_all(to)?;
    }

    for entry_result in fs::read_dir(from)? {
        let entry = entry_result?;
        let from_path = entry.path();
        let to_path = to.join(entry.file_name());

        if from_path.is_dir() {
            copy_dir_recursive(&from_path, &to_path)?;
        } else {
            fs::copy(&from_path, &to_path)?;
        }
    }
    Ok(())
}

// Function to split Paths
pub fn split_path(path: &str) -> Vec<String> {
    Path::new(path)
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect()
}

// Function to secure Paths
pub fn secure_path(path: &str) -> String {
    let mut normalized = PathBuf::new();
    for comp in Path::new(path).components() {
        match comp {
            Component::CurDir => continue,
            Component::ParentDir => {
                normalized.pop();
            }
            _ => normalized.push(comp),
        }
    }
    normalized.to_string_lossy().into_owned()
}

// Function to join paths
pub fn join_path(parts: Vec<String>) -> String {
    let mut path = PathBuf::new();
    for part in parts {
        path.push(part);
    }
    path.to_string_lossy().into_owned()
}
