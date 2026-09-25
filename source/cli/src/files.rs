use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub(crate) fn list_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            files.push(entry.path());
        }
    }

    files.sort();
    Ok(files)
}