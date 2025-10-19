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

/// Normalizes a path and checks for traversal attacks or absolute paths.
/// Returns a secure, normalized `PathBuf` or an error string.
pub fn secure_path(path: &str) -> Result<PathBuf, String> {
    #[cfg(unix)]
    let path = path.replace('\\', "/");

    #[cfg(not(unix))]
    let path = path.to_string();

    let original = Path::new(&path);

    // Reject absolute paths like /etc/passwd or C:\Windows
    if original.is_absolute() {
        return Err("Absolute paths are not allowed".into());
    }

    let mut normalized = PathBuf::new();

    for comp in original.components() {
        match comp {
            Component::CurDir => continue,
            Component::ParentDir => {
                // We disallow traversing above the root
                if !normalized.pop() {
                    return Err("Path attempts to escape base directory".into());
                }
            }
            Component::Normal(part) => normalized.push(part),
            _ => {
                return Err("Unsupported path component".into());
            }
        }
    }

    Ok(normalized)
}

// Function to join paths
pub fn join_path(parts: Vec<String>) -> String {
    let mut path = PathBuf::new();
    for part in parts {
        path.push(part);
    }
    path.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_path_parent_dir_at_start() {
        let path = "../foo/bar";
        let result = secure_path(path);
        assert!(
            result.is_err(),
            "Should reject path starting with parent dir"
        );
    }

    #[test]
    fn test_secure_path_only_parent_dirs() {
        let path = "../../..";
        let result = secure_path(path);
        assert!(result.is_err(), "Should reject path with only parent dirs");
    }

    #[test]
    fn test_split_path_simple() {
        let path = "foo/bar/baz";
        let parts = split_path(path);
        assert_eq!(parts, vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn test_split_path_absolute() {
        #[cfg(unix)]
        {
            let path = "/usr/local/bin";
            let parts = split_path(path);
            assert_eq!(parts, vec!["/", "usr", "local", "bin"]);
        }

        #[cfg(windows)]
        {
            // TODO
            // Maybe Windows here
        }
    }

    #[cfg(windows)]
    #[test]
    fn test_split_path_absolute2() {
        let path = r"C:\Program Files\Rust\bin";
        let parts = split_path(path);

        // Erwarte Laufwerksbuchstabe als erstes Element
        assert_eq!(parts[0], "C:");

        // RootDir eventuell als zweites, abhängig von split_path-Implementierung
        // Hier mal ignorieren, teste nur den Rest
        let rest = if parts.len() > 1 && parts[1] == "\\" {
            &parts[2..]
        } else {
            &parts[1..]
        };

        assert_eq!(rest, ["Program Files", "Rust", "bin"]);
    }

    #[test]
    fn test_split_path_with_dots() {
        let path = "./foo/../bar";
        let parts = split_path(path);
        // Split only, does not resolve
        assert!(parts.contains(&"..".to_string()));
    }

    #[test]
    fn test_join_path_basic() {
        let parts = vec!["foo".into(), "bar".into(), "baz".into()];
        let path = join_path(parts);
        assert!(path.ends_with("foo/bar/baz") || path.ends_with("foo\\bar\\baz"));
    }

    #[test]
    fn test_join_path_with_absolute() {
        #[cfg(unix)]
        let parts = vec!["/".into(), "usr".into(), "bin".into()];
        #[cfg(windows)]
        let parts = vec!["C:\\".into(), "Windows".into(), "System32".into()];

        let path = join_path(parts);
        #[cfg(unix)]
        assert_eq!(path, "/usr/bin");
        #[cfg(windows)]
        assert!(path.contains("Windows\\System32"));
    }

    use std::path::PathBuf;

    #[test]
    fn test_secure_path_basic() {
        let path = "foo/bar/baz";
        let result = secure_path(path).expect("Should be secure");
        assert_eq!(result, PathBuf::from("foo/bar/baz"));
    }

    #[test]
    fn test_secure_path_removes_current_dir() {
        let path = "foo/./bar";
        let result = secure_path(path).expect("Should normalize");
        assert_eq!(result, PathBuf::from("foo/bar"));
    }

    #[test]
    fn test_secure_path_removes_parent_dir() {
        let path = "foo/bar/../baz";
        let result = secure_path(path).expect("Should normalize");
        assert_eq!(result, PathBuf::from("foo/baz"));
    }

    #[test]
    fn test_secure_path_multiple_parent_dirs() {
        let path = "foo/bar/baz/../../qux";
        let result = secure_path(path).expect("Should normalize");
        assert_eq!(result, PathBuf::from("foo/qux"));
    }

    #[test]
    fn test_secure_path_with_absolute_path() {
        #[cfg(unix)]
        {
            let path = "/foo/bar/../baz";
            let result = secure_path(path);
            assert!(result.is_err()); // Absolute Pfade sind unsicher
        }

        #[cfg(windows)]
        {
            let path = r"C:\foo\bar\..\baz";
            let result = secure_path(path);
            assert!(result.is_err()); // Auch unter Windows: absolut = Fehler
        }
    }

    #[test]
    fn test_secure_path_with_mixed_separators() {
        let path = r"foo\bar/../baz";
        let result = secure_path(path).expect("Should normalize mixed separators");

        #[cfg(windows)]
        assert_eq!(result, PathBuf::from(r"foo\baz"));

        #[cfg(unix)]
        {
            assert_eq!(result, PathBuf::from("foo/baz"));

            assert_eq!(
                result.components().collect::<Vec<_>>(),
                PathBuf::from("foo/baz").components().collect::<Vec<_>>()
            );
        }
    }
}
