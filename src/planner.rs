use crate::agent::Agent;
use crate::content::{self, ContentScope};
use std::path::{Path, PathBuf};

/// An action the executor should perform.
#[derive(Debug, Clone)]
pub enum InstallAction {
    CreateDir {
        path: PathBuf,
    },
    WriteFile {
        path: PathBuf,
        content: String,
        overwrite: bool,
        manifest_base: PathBuf,
    },
    SkipExisting {
        path: PathBuf,
    },
    ManualInstruction {
        agent_label: String,
        filename: String,
        #[allow(dead_code)]
        content: String,
    },
    WriteManifest {
        base_dir: PathBuf,
        agent_name: String,
    },
    AppendGitignore {
        cwd: PathBuf,
        snippet: String,
    },
    /// Append (or replace) a named section in a single consolidated file.
    /// Used by agents that merge all rules into one file (Antigravity's GEMINI.md, TRAE's project_rules.md).
    AppendToFile {
        path: PathBuf,
        section_header: String,
        content: String,
    },
    CopyTemplate {
        dest: PathBuf,
        content: String,
        overwrite: bool,
    },
}

/// Plan workspace file installation for an agent.
pub fn plan_install(agent: &dyn Agent, cwd: &Path, force: bool) -> Vec<InstallAction> {
    let workspace_dir = agent.workspace_dir(cwd);
    let workflow_dir = agent.workflow_dir(cwd);
    let workspace_files = content::by_scope(ContentScope::Workspace);
    let consolidates = agent.consolidates_to_single_file();
    let mut actions = Vec::new();

    actions.push(InstallAction::CreateDir { path: workspace_dir.clone() });
    if let Some(ref wf_dir) = workflow_dir {
        actions.push(InstallAction::CreateDir { path: wf_dir.clone() });
    }

    for file in &workspace_files {
        let output = agent.transform_workspace(file);

        // Route to workflow_dir when agent has one and file is a workflow file.
        let target_dir = if workflow_dir.is_some()
            && matches!(
                file.name.as_str(),
                "prep-review.md" | "pack-materials.md" | "project-context.md"
            ) {
            workflow_dir.as_ref().unwrap().clone()
        } else {
            workspace_dir.clone()
        };

        if consolidates {
            // Use AppendToFile — section header is the original file name (without .md).
            let section = file.name.trim_end_matches(".md").to_string();
            actions.push(InstallAction::AppendToFile {
                path: target_dir.join(&output.filename),
                section_header: section,
                content: output.content,
            });
        } else {
            let dest = target_dir.join(&output.filename);
            if dest.exists() && !force {
                actions.push(InstallAction::SkipExisting { path: dest });
            } else {
                actions.push(InstallAction::WriteFile {
                    path: dest,
                    content: output.content,
                    overwrite: force,
                    manifest_base: target_dir,
                });
            }
        }
    }

    actions.push(InstallAction::WriteManifest {
        base_dir: workspace_dir,
        agent_name: agent.name().to_string(),
    });

    actions
}

/// Plan global file installation for an agent.
pub fn plan_install_global(agent: &dyn Agent, force: bool) -> Vec<InstallAction> {
    let global_files = content::by_scope(ContentScope::Global);
    let mut actions = Vec::new();

    match agent.global_dir() {
        Some(global_dir) => {
            actions.push(InstallAction::CreateDir {
                path: global_dir.clone(),
            });

            for file in &global_files {
                let output = agent.transform_global(file);
                let dest = global_dir.join(&output.filename);

                // When multiple files map to the same destination (e.g. GEMINI.md),
                // use AppendToFile with per-file section headers instead of WriteFile.
                let is_consolidated = global_files.iter().any(|f| {
                    f.name != file.name
                        && agent.transform_global(f).filename == output.filename
                });

                if is_consolidated {
                    let section = file.name.trim_end_matches(".md").to_string();
                    actions.push(InstallAction::AppendToFile {
                        path: dest,
                        section_header: section,
                        content: output.content,
                    });
                } else if dest.exists() && !force {
                    actions.push(InstallAction::SkipExisting { path: dest });
                } else {
                    actions.push(InstallAction::WriteFile {
                        path: dest,
                        content: output.content,
                        overwrite: force,
                        manifest_base: global_dir.clone(),
                    });
                }
            }

            actions.push(InstallAction::WriteManifest {
                base_dir: global_dir,
                agent_name: agent.name().to_string(),
            });
        }
        None => {
            for file in &global_files {
                let output = agent.transform_global(file);
                actions.push(InstallAction::ManualInstruction {
                    agent_label: agent.label().to_string(),
                    filename: output.filename,
                    content: output.content,
                });
            }
        }
    }

    actions
}

/// Plan templates installation (archignore, gitignore snippet).
pub fn plan_templates(cwd: &Path, force: bool) -> Vec<InstallAction> {
    let mut actions = Vec::new();

    // .archignore
    if let Some(archignore) = content::by_name("archignore") {
        let dest = cwd.join(".archignore");
        if dest.exists() && !force {
            actions.push(InstallAction::SkipExisting { path: dest });
        } else {
            actions.push(InstallAction::CopyTemplate {
                dest,
                content: archignore.raw.to_string(),
                overwrite: force,
            });
        }
    }

    // gitignore snippet
    if let Some(snippet) = content::by_name("gitignore-snippet.txt") {
        actions.push(InstallAction::AppendGitignore {
            cwd: cwd.to_path_buf(),
            snippet: snippet.raw.to_string(),
        });
    }

    actions
}

/// Plan full installation: workspace + global + templates.
pub fn plan_install_all(agent: &dyn Agent, cwd: &Path, force: bool) -> Vec<InstallAction> {
    let mut actions = Vec::new();
    actions.extend(plan_install(agent, cwd, force));
    actions.extend(plan_install_global(agent, force));
    actions.extend(plan_templates(cwd, force));
    actions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::kiro::Kiro;
    use crate::agent::cursor::Cursor;
    use std::path::PathBuf;

    #[test]
    fn plan_install_workspace_creates_dir_and_files() {
        let kiro = Kiro::new();
        let cwd = Path::new("/fake/project");
        let actions = plan_install(&kiro, cwd, false);

        // First action should be CreateDir
        assert!(matches!(&actions[0], InstallAction::CreateDir { .. }));

        // Should have 4 WriteFile actions (workspace has 4 files)
        let write_count = actions.iter().filter(|a| matches!(a, InstallAction::WriteFile { .. })).count();
        assert_eq!(write_count, 4);

        // Should have a WriteManifest
        assert!(actions.iter().any(|a| matches!(a, InstallAction::WriteManifest { .. })));
    }

    #[test]
    fn plan_install_global_cursor_produces_manual_instructions() {
        let cursor = Cursor::new();
        let actions = plan_install_global(&cursor, false);

        let manual_count = actions.iter().filter(|a| matches!(a, InstallAction::ManualInstruction { .. })).count();
        assert_eq!(manual_count, 6); // 6 global files
    }

    #[test]
    fn plan_install_global_kiro_produces_write_files() {
        let mut kiro = Kiro::new();
        kiro.home.override_path = Some(PathBuf::from("/fake/home"));
        let actions = plan_install_global(&kiro, false);

        let write_count = actions.iter().filter(|a| matches!(a, InstallAction::WriteFile { .. })).count();
        assert_eq!(write_count, 6); // 6 global files
    }

    #[test]
    fn plan_install_force_sets_overwrite() {
        let mut kiro = Kiro::new();
        kiro.home.override_path = Some(PathBuf::from("/fake/home"));
        let actions = plan_install_global(&kiro, true);

        for action in &actions {
            if let InstallAction::WriteFile { overwrite, .. } = action {
                assert!(overwrite);
            }
        }
    }

    #[test]
    fn plan_install_all_combines_all_scopes() {
        let mut kiro = Kiro::new();
        kiro.home.override_path = Some(PathBuf::from("/fake/home"));
        let cwd = Path::new("/fake/project");
        let actions = plan_install_all(&kiro, cwd, false);

        // Should have actions from workspace, global, and templates
        let create_dir_count = actions.iter().filter(|a| matches!(a, InstallAction::CreateDir { .. })).count();
        assert!(create_dir_count >= 2); // workspace dir + global dir

        let write_count = actions.iter().filter(|a| matches!(a, InstallAction::WriteFile { .. })).count();
        assert_eq!(write_count, 10); // 4 workspace + 6 global
    }
}
