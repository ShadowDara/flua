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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_secure_path_basic() {
        let path = "foo/bar/baz";
        assert_eq!(secure_path(path), "foo/bar/baz");
    }

    #[test]
    fn test_secure_path_removes_current_dir() {
        let path = "foo/./bar";
        assert_eq!(secure_path(path), "foo/bar");
    }

    #[test]
    fn test_secure_path_removes_parent_dir() {
        let path = "foo/bar/../baz";
        assert_eq!(secure_path(path), "foo/baz");
    }

    #[test]
    fn test_secure_path_multiple_parent_dirs() {
        let path = "foo/bar/baz/../../qux";
        assert_eq!(secure_path(path), "foo/qux");
    }

    #[test]
    fn test_secure_path_parent_dir_at_start() {
        // ../ at start pops nothing, path stays empty or no error
        let path = "../foo/bar";
        assert_eq!(secure_path(path), "foo/bar");
    }

    #[test]
    fn test_secure_path_only_parent_dirs() {
        let path = "../../..";
        assert_eq!(secure_path(path), "");
    }

    #[test]
    fn test_secure_path_with_absolute_path() {
        #[cfg(unix)]
        {
            let path = "/foo/bar/../baz";
            // normalized absolute path
            assert_eq!(secure_path(path), "/foo/baz");
        }
        #[cfg(windows)]
        {
            let path = r"C:\foo\bar\..\baz";
            assert_eq!(secure_path(path), r"C:\foo\baz");
        }
    }

    #[test]
    fn test_secure_path_with_mixed_separators() {
        let path = r"foo\bar/../baz";
        // on Windows, Path handles both separators, so normalized should be foo\baz
        // on Unix, backslash is a valid char, so probably no normalization on that part
        #[cfg(windows)]
        assert_eq!(secure_path(path), r"foo\baz");
        #[cfg(unix)]
        assert_eq!(secure_path(path), r"foo\baz"); // note: backslash remains
    }
}
