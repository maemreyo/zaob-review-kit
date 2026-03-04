use crate::error::ZrkError;
use std::fs;
use std::path::Path;

/// Ensure a directory exists, creating it recursively if needed.
pub fn ensure_dir(path: &Path) -> Result<(), ZrkError> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Write a file safely — creates parent dirs if needed.
pub fn write_file_safe(path: &Path, content: &str) -> Result<(), ZrkError> {
    if let Some(parent) = path.parent() {
        ensure_dir(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}

/// Append gitignore rules if not already present.
pub fn append_gitignore(cwd: &Path, snippet: &str) -> Result<bool, ZrkError> {
    let gitignore_path = cwd.join(".gitignore");
    let marker = "# zrk review materials";

    if gitignore_path.exists() {
        let existing = fs::read_to_string(&gitignore_path)?;
        if existing.contains(marker) {
            return Ok(false);
        }
        let separator = if existing.ends_with('\n') { "\n" } else { "\n\n" };
        fs::write(&gitignore_path, format!("{}{}{}", existing, separator, snippet))?;
    } else {
        fs::write(&gitignore_path, snippet)?;
    }
    Ok(true)
}
