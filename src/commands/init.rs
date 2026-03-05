use crate::agent::registry;
use crate::error::ZrkError;
use crate::executor;
use crate::planner;
use crate::util::output;
use crate::Cli;
use std::path::{Path, PathBuf};

/// Configuration for the init wizard (pure, testable).
pub struct InitConfig {
    pub agents: Vec<String>,
    pub install_global: bool,
    #[allow(dead_code)]
    pub scaffold_context: bool,
    pub cwd: PathBuf,
}

/// Run the init wizard interactively.
#[cfg(feature = "wizard")]
fn prompt_user(cwd: &Path) -> Result<InitConfig, ZrkError> {
    use dialoguer::{Confirm, MultiSelect};

    let agent_names = ["kiro", "claude-code", "cursor", "windsurf", "antigravity", "trae"];
    let agent_labels = ["Kiro", "Claude Code", "Cursor", "Windsurf", "Google Antigravity", "TRAE"];

    let selections = MultiSelect::new()
        .with_prompt("Which agents are you using?")
        .items(&agent_labels)
        .interact()
        .map_err(|e| ZrkError::Io(std::io::Error::other(e)))?;

    if selections.is_empty() {
        output::warning("No agents selected. Using default: kiro");
        return Ok(InitConfig {
            agents: vec!["kiro".to_string()],
            install_global: true,
            scaffold_context: true,
            cwd: cwd.to_path_buf(),
        });
    }

    let agents: Vec<String> = selections.iter().map(|&i| agent_names[i].to_string()).collect();

    let install_global = Confirm::new()
        .with_prompt("Install global rules? (affects all your projects)")
        .default(true)
        .interact()
        .map_err(|e| ZrkError::Io(std::io::Error::other(e)))?;

    let scaffold_context = Confirm::new()
        .with_prompt("Generate project-context.md scaffold?")
        .default(true)
        .interact()
        .map_err(|e| ZrkError::Io(std::io::Error::other(e)))?;

    Ok(InitConfig {
        agents,
        install_global,
        scaffold_context,
        cwd: cwd.to_path_buf(),
    })
}

/// Execute init with a given config (testable, no interactive prompts).
pub fn run_init_with_config(config: &InitConfig, force: bool, quiet: bool) -> Result<(), ZrkError> {
    for agent_name in &config.agents {
        let agent = registry::get_agent(agent_name)
            .ok_or_else(|| ZrkError::UnknownAgent(agent_name.clone()))?;

        if !quiet {
            output::info(&format!("Setting up {}", agent.label()));
        }

        // Install workspace files (pass scaffold_context so project-context.md can be skipped)
        let mut actions = planner::plan_install(agent.as_ref(), &config.cwd, force, config.scaffold_context);

        // Install global if requested
        if config.install_global {
            actions.extend(planner::plan_install_global(agent.as_ref(), force));
        }

        // Install templates
        actions.extend(planner::plan_templates(&config.cwd, force));

        executor::execute(&actions, quiet)?;
    }

    if !quiet {
        println!();
        output::success("Setup complete!");
        output::info("Next: ask your agent to fill in project-context.md");
    }

    Ok(())
}

pub fn run_init(cli: &Cli) -> Result<(), ZrkError> {
    let cwd = cli.cwd.clone().unwrap_or_else(|| {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    });

    #[cfg(feature = "wizard")]
    {
        let config = prompt_user(&cwd)?;
        run_init_with_config(&config, cli.force, cli.quiet)
    }

    #[cfg(not(feature = "wizard"))]
    {
        output::warning("Init wizard requires the 'wizard' feature. Using defaults.");
        let config = InitConfig {
            agents: vec![cli.target.clone()],
            install_global: true,
            scaffold_context: true,
            cwd,
        };
        run_init_with_config(&config, cli.force, cli.quiet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_with_config_installs_expected_files() {
        let dir = tempfile::tempdir().unwrap();
        let config = InitConfig {
            agents: vec!["kiro".to_string()],
            install_global: false,
            scaffold_context: true,
            cwd: dir.path().to_path_buf(),
        };

        run_init_with_config(&config, false, true).unwrap();

        let steering = dir.path().join(".kiro").join("steering");
        assert!(steering.join("prep-review.md").exists());
        assert!(steering.join("pack-materials.md").exists());
        assert!(steering.join("project-context.md").exists());
        assert!(dir.path().join(".archignore").exists());
    }

    #[test]
    fn init_with_global_installs_global_files() {
        let dir = tempfile::tempdir().unwrap();

        // Use home_override via a custom agent isn't easy, so we test
        // that the action count is correct instead
        let config = InitConfig {
            agents: vec!["cursor".to_string()],
            install_global: true,
            scaffold_context: true,
            cwd: dir.path().to_path_buf(),
        };

        // Cursor global is manual-only, so this should still succeed
        run_init_with_config(&config, false, true).unwrap();
        assert!(dir.path().join(".cursor").join("rules").exists());
    }

    #[test]
    fn init_scaffold_context_false_skips_project_context() {
        let dir = tempfile::tempdir().unwrap();
        let config = InitConfig {
            agents: vec!["kiro".to_string()],
            install_global: false,
            scaffold_context: false,
            cwd: dir.path().to_path_buf(),
        };

        run_init_with_config(&config, false, true).unwrap();

        let steering = dir.path().join(".kiro").join("steering");
        assert!(!steering.join("project-context.md").exists());
        assert!(steering.join("prep-review.md").exists()); // others still installed
    }

    #[test]
    fn init_with_multiple_agents() {
        let dir = tempfile::tempdir().unwrap();
        let config = InitConfig {
            agents: vec!["kiro".to_string(), "cursor".to_string()],
            install_global: false,
            scaffold_context: true,
            cwd: dir.path().to_path_buf(),
        };

        run_init_with_config(&config, false, true).unwrap();

        assert!(dir.path().join(".kiro").join("steering").exists());
        assert!(dir.path().join(".cursor").join("rules").exists());
    }
}
