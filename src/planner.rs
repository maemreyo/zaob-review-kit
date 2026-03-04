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
    CopyTemplate {
        dest: PathBuf,
        content: String,
        overwrite: bool,
    },
}

/// Plan workspace file installation for an agent.
pub fn plan_install(agent: &dyn Agent, cwd: &Path, force: bool) -> Vec<InstallAction> {
    let workspace_dir = agent.workspace_dir(cwd);
    let workspace_files = content::by_scope(ContentScope::Workspace);
    let mut actions = Vec::new();

    actions.push(InstallAction::CreateDir {
        path: workspace_dir.clone(),
    });

    for file in &workspace_files {
        let output = agent.transform_workspace(file);
        let dest = workspace_dir.join(&output.filename);

        if dest.exists() && !force {
            actions.push(InstallAction::SkipExisting { path: dest });
        } else {
            actions.push(InstallAction::WriteFile {
                path: dest,
                content: output.content,
                overwrite: force,
            });
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

                if dest.exists() && !force {
                    actions.push(InstallAction::SkipExisting { path: dest });
                } else {
                    actions.push(InstallAction::WriteFile {
                        path: dest,
                        content: output.content,
                        overwrite: force,
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

        // Should have 3 WriteFile actions (workspace has 3 files)
        let write_count = actions.iter().filter(|a| matches!(a, InstallAction::WriteFile { .. })).count();
        assert_eq!(write_count, 3);

        // Should have a WriteManifest
        assert!(actions.iter().any(|a| matches!(a, InstallAction::WriteManifest { .. })));
    }

    #[test]
    fn plan_install_global_cursor_produces_manual_instructions() {
        let cursor = Cursor::new();
        let actions = plan_install_global(&cursor, false);

        let manual_count = actions.iter().filter(|a| matches!(a, InstallAction::ManualInstruction { .. })).count();
        assert_eq!(manual_count, 4); // 4 global files
    }

    #[test]
    fn plan_install_global_kiro_produces_write_files() {
        let mut kiro = Kiro::new();
        kiro.home_override = Some(PathBuf::from("/fake/home"));
        let actions = plan_install_global(&kiro, false);

        let write_count = actions.iter().filter(|a| matches!(a, InstallAction::WriteFile { .. })).count();
        assert_eq!(write_count, 4); // 4 global files
    }

    #[test]
    fn plan_install_force_sets_overwrite() {
        let mut kiro = Kiro::new();
        kiro.home_override = Some(PathBuf::from("/fake/home"));
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
        kiro.home_override = Some(PathBuf::from("/fake/home"));
        let cwd = Path::new("/fake/project");
        let actions = plan_install_all(&kiro, cwd, false);

        // Should have actions from workspace, global, and templates
        let create_dir_count = actions.iter().filter(|a| matches!(a, InstallAction::CreateDir { .. })).count();
        assert!(create_dir_count >= 2); // workspace dir + global dir

        let write_count = actions.iter().filter(|a| matches!(a, InstallAction::WriteFile { .. })).count();
        assert_eq!(write_count, 7); // 3 workspace + 4 global
    }
}
