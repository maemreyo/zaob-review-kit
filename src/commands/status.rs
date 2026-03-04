use crate::agent::registry;
use crate::error::ZrkError;
use crate::manifest::{sha256_hex, Manifest};
use crate::util::output;
use crate::Cli;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub enum FileState {
    Installed,
    Modified,
    Missing,
    NoManifest,
}

pub struct FileStatus {
    pub path: String,
    pub state: FileState,
}

pub fn check_status(base_dir: &Path) -> Vec<FileStatus> {
    let manifest = match Manifest::load(base_dir) {
        Ok(Some(m)) => m,
        Ok(None) => {
            return vec![FileStatus {
                path: base_dir.display().to_string(),
                state: FileState::NoManifest,
            }];
        }
        Err(_) => {
            return vec![FileStatus {
                path: base_dir.display().to_string(),
                state: FileState::NoManifest,
            }];
        }
    };

    let mut results = Vec::new();
    for (filename, expected_hash) in &manifest.files {
        let file_path = base_dir.join(filename);
        if !file_path.exists() {
            results.push(FileStatus {
                path: filename.clone(),
                state: FileState::Missing,
            });
        } else {
            let content = std::fs::read_to_string(&file_path).unwrap_or_default();
            let actual_hash = sha256_hex(&content);
            if actual_hash == *expected_hash {
                results.push(FileStatus {
                    path: filename.clone(),
                    state: FileState::Installed,
                });
            } else {
                results.push(FileStatus {
                    path: filename.clone(),
                    state: FileState::Modified,
                });
            }
        }
    }
    results
}

fn display_status(label: &str, dir_label: &str, statuses: &[FileStatus], quiet: bool) {
    if quiet {
        return;
    }
    println!("\n  {} ({})", label, dir_label);
    for status in statuses {
        match status.state {
            FileState::Installed => output::success(&format!("  {}", status.path)),
            FileState::Modified => output::warning(&format!("  {} (modified)", status.path)),
            FileState::Missing => {
                eprintln!("  \u{2717}   {} (missing)", status.path);
            }
            FileState::NoManifest => output::neutral(&format!("  {} (no manifest)", status.path)),
        }
    }
}

pub fn run_status(cli: &Cli) -> Result<(), ZrkError> {
    let cwd = cli.cwd.clone().unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let agents = if cli.all_targets {
        registry::all_agents()
    } else {
        match registry::get_agent(&cli.target) {
            Some(agent) => vec![agent],
            None => return Err(ZrkError::UnknownAgent(cli.target.clone())),
        }
    };

    for agent in &agents {
        if !cli.quiet {
            println!("\n{}", agent.label());
        }

        // Check workspace
        let workspace_dir = agent.workspace_dir(&cwd);
        if workspace_dir.exists() {
            let statuses = check_status(&workspace_dir);
            display_status("Workspace", &workspace_dir.display().to_string(), &statuses, cli.quiet);
        } else if !cli.quiet {
            output::neutral(&format!("  Workspace: not installed ({})", workspace_dir.display()));
        }

        // Check global
        match agent.global_dir() {
            Some(global_dir) => {
                if global_dir.exists() {
                    let statuses = check_status(&global_dir);
                    display_status("Global", &global_dir.display().to_string(), &statuses, cli.quiet);
                } else if !cli.quiet {
                    output::neutral(&format!("  Global: not installed ({})", global_dir.display()));
                }
            }
            None => {
                if !cli.quiet {
                    output::warning("  Global: manual setup required");
                }
            }
        }
    }

    // Check templates
    if !cli.quiet {
        println!();
        let archignore = cwd.join(".archignore");
        if archignore.exists() {
            output::success(".archignore");
        } else {
            output::neutral(".archignore (not installed)");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Manifest;

    #[test]
    fn status_freshly_installed_all_installed() {
        let dir = tempfile::tempdir().unwrap();
        let content = "# Test content";

        // Write a file and create a matching manifest
        std::fs::write(dir.path().join("test.md"), content).unwrap();
        let mut manifest = Manifest::new("kiro");
        manifest.add_file("test.md", content);
        manifest.save(dir.path()).unwrap();

        let statuses = check_status(dir.path());
        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].state, FileState::Installed);
    }

    #[test]
    fn status_deleted_file_is_missing() {
        let dir = tempfile::tempdir().unwrap();

        // Create manifest referencing a file that doesn't exist
        let mut manifest = Manifest::new("kiro");
        manifest.add_file("missing.md", "content");
        manifest.save(dir.path()).unwrap();

        let statuses = check_status(dir.path());
        assert_eq!(statuses[0].state, FileState::Missing);
    }

    #[test]
    fn status_modified_file_detected() {
        let dir = tempfile::tempdir().unwrap();

        // Create manifest with one hash, but file has different content
        let mut manifest = Manifest::new("kiro");
        manifest.add_file("test.md", "original content");
        manifest.save(dir.path()).unwrap();

        std::fs::write(dir.path().join("test.md"), "modified content").unwrap();

        let statuses = check_status(dir.path());
        assert_eq!(statuses[0].state, FileState::Modified);
    }

    #[test]
    fn status_no_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let statuses = check_status(dir.path());
        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].state, FileState::NoManifest);
    }
}
