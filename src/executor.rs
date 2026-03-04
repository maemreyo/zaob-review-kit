use crate::error::ZrkError;
use crate::manifest::Manifest;
use crate::planner::InstallAction;
use crate::util::fs::{append_gitignore, ensure_dir, write_file_safe};
use crate::util::output;

/// Summary of execution results.
#[derive(Debug, Default)]
pub struct InstallSummary {
    pub installed: usize,
    pub skipped: usize,
    pub manual: usize,
    pub errors: usize,
}

impl InstallSummary {
    pub fn merge(&mut self, other: &InstallSummary) {
        self.installed += other.installed;
        self.skipped += other.skipped;
        self.manual += other.manual;
        self.errors += other.errors;
    }
}

/// Execute a list of install actions, writing files to disk.
pub fn execute(actions: &[InstallAction], quiet: bool) -> Result<InstallSummary, ZrkError> {
    let mut summary = InstallSummary::default();
    let mut manifest: Option<(std::path::PathBuf, Manifest)> = None;

    for action in actions {
        match action {
            InstallAction::CreateDir { path } => {
                ensure_dir(path)?;
            }
            InstallAction::WriteFile { path, content, overwrite: _ } => {
                write_file_safe(path, content)?;
                if !quiet {
                    output::success(&format!("Installed {}", path.display()));
                }
                summary.installed += 1;

                // Track in manifest
                if let Some((_, ref mut m)) = manifest {
                    if let Some(filename) = path.file_name() {
                        m.add_file(&filename.to_string_lossy(), content);
                    }
                }
            }
            InstallAction::SkipExisting { path } => {
                if !quiet {
                    output::warning(&format!("Skipped (exists) {}", path.display()));
                }
                summary.skipped += 1;
            }
            InstallAction::ManualInstruction { agent_label, filename, content: _ } => {
                if !quiet {
                    output::warning(&format!(
                        "{}: '{}' requires manual setup (global rules are UI-only)",
                        agent_label, filename
                    ));
                }
                summary.manual += 1;
            }
            InstallAction::WriteManifest { base_dir, agent_name } => {
                let mut m = Manifest::new(agent_name);
                // Re-collect file hashes from what we just wrote
                if let Some((ref prev_dir, ref prev_manifest)) = manifest {
                    if prev_dir == base_dir {
                        m = prev_manifest.clone();
                    }
                }
                m.save(base_dir).map_err(ZrkError::Io)?;
                manifest = Some((base_dir.clone(), m));
            }
            InstallAction::AppendGitignore { cwd, snippet } => {
                match append_gitignore(cwd, snippet) {
                    Ok(true) => {
                        if !quiet {
                            output::success("Updated .gitignore with review material patterns");
                        }
                    }
                    Ok(false) => {
                        if !quiet {
                            output::neutral(".gitignore already has zrk patterns");
                        }
                    }
                    Err(e) => {
                        if !quiet {
                            output::warning(&format!("Could not update .gitignore: {}", e));
                        }
                    }
                }
            }
            InstallAction::CopyTemplate { dest, content, overwrite: _ } => {
                write_file_safe(dest, content)?;
                if !quiet {
                    output::success(&format!("Installed {}", dest.display()));
                }
                summary.installed += 1;
            }
        }
    }

    Ok(summary)
}

/// Display what actions would be taken without executing them.
pub fn dry_run_display(actions: &[InstallAction]) {
    for action in actions {
        match action {
            InstallAction::CreateDir { path } => {
                output::info(&format!("Would create directory: {}", path.display()));
            }
            InstallAction::WriteFile { path, overwrite, .. } => {
                let verb = if *overwrite { "overwrite" } else { "create" };
                output::info(&format!("Would {} file: {}", verb, path.display()));
            }
            InstallAction::SkipExisting { path } => {
                output::neutral(&format!("Would skip (exists): {}", path.display()));
            }
            InstallAction::ManualInstruction { agent_label, filename, .. } => {
                output::warning(&format!(
                    "Would print manual instructions for {}: {}",
                    agent_label, filename
                ));
            }
            InstallAction::WriteManifest { base_dir, .. } => {
                output::info(&format!("Would write manifest in: {}", base_dir.display()));
            }
            InstallAction::AppendGitignore { cwd, .. } => {
                output::info(&format!("Would update .gitignore in: {}", cwd.display()));
            }
            InstallAction::CopyTemplate { dest, overwrite, .. } => {
                let verb = if *overwrite { "overwrite" } else { "create" };
                output::info(&format!("Would {} template: {}", verb, dest.display()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planner::InstallAction;
    #[test]
    fn executor_creates_files_on_disk() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("subdir").join("test.md");

        let actions = vec![
            InstallAction::CreateDir { path: dir.path().join("subdir") },
            InstallAction::WriteFile {
                path: file_path.clone(),
                content: "# Test".to_string(),
                overwrite: false,
            },
        ];

        let summary = execute(&actions, true).unwrap();
        assert_eq!(summary.installed, 1);
        assert!(file_path.exists());
        assert_eq!(std::fs::read_to_string(&file_path).unwrap(), "# Test");
    }

    #[test]
    fn executor_skips_existing() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("existing.md");
        std::fs::write(&file_path, "old content").unwrap();

        let actions = vec![InstallAction::SkipExisting {
            path: file_path.clone(),
        }];

        let summary = execute(&actions, true).unwrap();
        assert_eq!(summary.skipped, 1);
        assert_eq!(std::fs::read_to_string(&file_path).unwrap(), "old content");
    }

    #[test]
    fn executor_overwrites_when_forced() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("overwrite.md");
        std::fs::write(&file_path, "old content").unwrap();

        let actions = vec![InstallAction::WriteFile {
            path: file_path.clone(),
            content: "new content".to_string(),
            overwrite: true,
        }];

        let summary = execute(&actions, true).unwrap();
        assert_eq!(summary.installed, 1);
        assert_eq!(std::fs::read_to_string(&file_path).unwrap(), "new content");
    }

    #[test]
    fn executor_counts_manual_instructions() {
        let actions = vec![InstallAction::ManualInstruction {
            agent_label: "Cursor".to_string(),
            filename: "test.mdc".to_string(),
            content: "content".to_string(),
        }];

        let summary = execute(&actions, true).unwrap();
        assert_eq!(summary.manual, 1);
    }

    #[test]
    fn dry_run_creates_no_files() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("should-not-exist.md");

        let actions = vec![InstallAction::WriteFile {
            path: file_path.clone(),
            content: "content".to_string(),
            overwrite: false,
        }];

        dry_run_display(&actions);
        assert!(!file_path.exists());
    }
}
