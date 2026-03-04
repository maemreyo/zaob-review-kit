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
    // Accumulate file hashes per manifest scope (base_dir → Manifest).
    // Populated by WriteFile, consumed by WriteManifest.
    let mut pending: std::collections::HashMap<std::path::PathBuf, Manifest> =
        std::collections::HashMap::new();

    for action in actions {
        match action {
            InstallAction::CreateDir { path } => {
                ensure_dir(path)?;
            }
            InstallAction::WriteFile { path, content, overwrite: _, manifest_base } => {
                write_file_safe(path, content)?;
                if !quiet {
                    output::success(&format!("Installed {}", path.display()));
                }
                summary.installed += 1;

                // Accumulate into the correct manifest scope.
                if let Some(filename) = path.file_name() {
                    pending
                        .entry(manifest_base.clone())
                        .or_insert_with(|| Manifest::new(""))
                        .add_file(&filename.to_string_lossy(), content);
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
                // Take accumulated hashes for this scope, set correct agent name, save.
                let mut m = pending.remove(base_dir).unwrap_or_else(|| Manifest::new(agent_name));
                m.agent = agent_name.clone();
                m.save(base_dir).map_err(ZrkError::Io)?;
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
        let subdir = dir.path().join("subdir");
        let file_path = subdir.join("test.md");

        let actions = vec![
            InstallAction::CreateDir { path: subdir.clone() },
            InstallAction::WriteFile {
                path: file_path.clone(),
                content: "# Test".to_string(),
                overwrite: false,
                manifest_base: subdir.clone(),
            },
            InstallAction::WriteManifest {
                base_dir: subdir.clone(),
                agent_name: "kiro".to_string(),
            },
        ];

        let summary = execute(&actions, true).unwrap();
        assert_eq!(summary.installed, 1);
        assert!(file_path.exists());
        assert_eq!(std::fs::read_to_string(&file_path).unwrap(), "# Test");

        // Manifest must exist and contain the file's hash
        let manifest = crate::manifest::Manifest::load(&subdir).unwrap().unwrap();
        assert!(manifest.files.contains_key("test.md"));
        assert_eq!(manifest.agent, "kiro");
    }

    #[test]
    fn executor_manifest_has_correct_hashes() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().to_path_buf();
        let content = "# Review Roles";
        let file_path = base.join("review-roles.md");

        let actions = vec![
            InstallAction::WriteFile {
                path: file_path.clone(),
                content: content.to_string(),
                overwrite: false,
                manifest_base: base.clone(),
            },
            InstallAction::WriteManifest {
                base_dir: base.clone(),
                agent_name: "kiro".to_string(),
            },
        ];

        execute(&actions, true).unwrap();

        let manifest = crate::manifest::Manifest::load(&base).unwrap().unwrap();
        let expected_hash = crate::manifest::sha256_hex(content);
        assert_eq!(manifest.files["review-roles.md"], expected_hash);
    }

    #[test]
    fn executor_two_scopes_produce_two_manifests() {
        let dir = tempfile::tempdir().unwrap();
        let workspace = dir.path().join("workspace");
        let global = dir.path().join("global");
        std::fs::create_dir_all(&workspace).unwrap();
        std::fs::create_dir_all(&global).unwrap();

        let actions = vec![
            InstallAction::WriteFile {
                path: workspace.join("prep-review.md"),
                content: "workspace content".to_string(),
                overwrite: false,
                manifest_base: workspace.clone(),
            },
            InstallAction::WriteFile {
                path: global.join("review-roles.md"),
                content: "global content".to_string(),
                overwrite: false,
                manifest_base: global.clone(),
            },
            InstallAction::WriteManifest {
                base_dir: workspace.clone(),
                agent_name: "kiro".to_string(),
            },
            InstallAction::WriteManifest {
                base_dir: global.clone(),
                agent_name: "kiro".to_string(),
            },
        ];

        execute(&actions, true).unwrap();

        let ws_manifest = crate::manifest::Manifest::load(&workspace).unwrap().unwrap();
        let gl_manifest = crate::manifest::Manifest::load(&global).unwrap().unwrap();
        assert!(ws_manifest.files.contains_key("prep-review.md"));
        assert!(gl_manifest.files.contains_key("review-roles.md"));
        assert!(!ws_manifest.files.contains_key("review-roles.md"));
        assert!(!gl_manifest.files.contains_key("prep-review.md"));
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
            manifest_base: dir.path().to_path_buf(),
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
            manifest_base: dir.path().to_path_buf(),
        }];

        dry_run_display(&actions);
        assert!(!file_path.exists());
    }
}
