use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::{ZipWriter, read::ZipArchive, write::FileOptions};

// Utility: Sicherheitsschranken für Pfade
pub fn validate_path(path: &str) -> Result<PathBuf, String> {
    let pb = PathBuf::from(path);
    if pb.is_absolute() {
        return Err("Absolute Pfade sind nicht erlaubt.".into());
    }

    if pb
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err("Pfad darf keine '..' enthalten.".into());
    }

    Ok(pb)
}

/// Entpackt sicher eine ZIP-Datei in ein Zielverzeichnis.
/// Verhindert Zip Slip (../../etc/passwd-Angriffe).
pub fn unzip_file(zip_path: &str, dest_dir: &str) -> io::Result<()> {
    let file = fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    let dest_path = Path::new(dest_dir);
    fs::create_dir_all(dest_path)?; // Erst erstellen!

    let dest_path = dest_path.canonicalize()?; // Danach canonicalisieren

    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i)?;
        let name = zip_file.name();

        // Sicherheits-Check: verhindern von "Zip Slip" (../../etc/passwd)
        let outpath = dest_path.join(name);

        // Wichtig: kein canonicalize auf nicht existierende Datei → nur `components` prüfen
        let outpath_clean = outpath
            .canonicalize()
            .or_else(|_| Ok::<PathBuf, io::Error>(clean_path(&outpath)))?;

        if !outpath_clean.starts_with(&dest_path) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Unsicherer Pfad im ZIP-Archiv: {}", name),
            ));
        }

        if zip_file.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut zip_file, &mut outfile)?;
        }
    }

    Ok(())
}

// Hilfsfunktion, um Pfade "normalisiert" zu bereinigen ohne auf das Dateisystem zuzugreifen
fn clean_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut result = PathBuf::new();
    for comp in path.as_ref().components() {
        match comp {
            std::path::Component::ParentDir => {
                result.pop();
            }
            std::path::Component::CurDir => {}
            other => result.push(other.as_os_str()),
        }
    }
    result
}

/// Verzeichnis rekursiv zippen
pub fn zip_dir(src_dir: &str, zip_path: &str) -> io::Result<()> {
    let file = fs::File::create(zip_path)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let src_path = Path::new(src_dir).canonicalize()?;

    for entry in WalkDir::new(&src_path).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        let name = path.strip_prefix(&src_path).unwrap();

        let name_str = name.to_string_lossy().replace("\\", "/"); // Kompatibilität mit Windows

        if path.is_file() {
            zip.start_file(name_str, options)?;
            let mut f = fs::File::open(path)?;
            io::copy(&mut f, &mut zip)?;
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(name_str, options)?;
        }
    }

    zip.finish()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_zip_and_unzip_directory() {
        let temp = assert_fs::TempDir::new().expect("Temp dir konnte nicht erstellt werden");

        // Quelle vorbereiten
        let src_dir = temp.child("source");
        src_dir
            .create_dir_all()
            .expect("Konnte Quellverzeichnis nicht erstellen");
        src_dir.child("test.txt").write_str("Hello, ZIP!").unwrap();
        src_dir.child("subdir").create_dir_all().unwrap();
        src_dir
            .child("subdir/test2.txt")
            .write_str("Nested file")
            .unwrap();

        // ZIP erstellen
        let zip_file = temp.child("archive.zip");
        zip_dir(
            src_dir.path().to_str().unwrap(),
            zip_file.path().to_str().unwrap(),
        )
        .expect("Konnte ZIP nicht erstellen");

        // Zielverzeichnis vorbereiten
        let dest_dir = temp.child("unzipped");

        // Entpacken
        unzip_file(
            zip_file.path().to_str().unwrap(),
            dest_dir.path().to_str().unwrap(),
        )
        .expect("Konnte ZIP nicht entpacken");

        // Überprüfen
        dest_dir
            .child("test.txt")
            .assert(predicates::str::contains("Hello, ZIP!"));
        dest_dir
            .child("subdir/test2.txt")
            .assert(predicates::str::contains("Nested file"));
    }

    #[test]
    fn test_unzip_with_zip_slip_attack_should_fail() {
        let temp = assert_fs::TempDir::new().unwrap();
        let zip_path = temp.child("evil.zip");
        let dest_dir = temp.child("unzipped");

        // ZIP mit bösem Eintrag manuell erzeugen
        {
            let file = fs::File::create(zip_path.path()).unwrap();
            let mut zip = ZipWriter::new(file);
            let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
            zip.start_file("../../evil.txt", options).unwrap();
            zip.write_all(b"malicious content").unwrap();
            zip.finish().unwrap();
        }

        let result = unzip_file(
            zip_path.path().to_str().unwrap(),
            dest_dir.path().to_str().unwrap(),
        );
        assert!(result.is_err());
    }
}
