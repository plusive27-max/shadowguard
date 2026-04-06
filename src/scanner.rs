use crate::error::ShadowGuardError;
use walkdir::WalkDir;
use std::path::PathBuf;

pub fn scan_directory(path: &str) -> Result<Vec<PathBuf>, ShadowGuardError> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            files.push(entry.into_path());
        }
    }
    Ok(files)
}
