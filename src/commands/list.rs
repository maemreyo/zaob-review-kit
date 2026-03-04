use crate::agent::registry;
use crate::content;
use crate::error::ZrkError;
use crate::Cli;

pub fn run_list(cli: &Cli) -> Result<(), ZrkError> {
    if !cli.quiet {
        println!("Supported agents:");
        for agent in registry::all_agents() {
            let global = match agent.global_dir() {
                Some(dir) => dir.display().to_string(),
                None => "(manual only)".to_string(),
            };
            println!("  {} ({})", agent.label(), agent.name());
            println!("    Global:    {}", global);
            println!("    Workspace: {}", agent.workspace_dir(std::path::Path::new(".")).display());
        }

        println!("\nContent files:");
        for file in content::all_content() {
            println!("  [{:?}] {}", file.scope, file.name);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::agent::registry;
    use crate::content;

    #[test]
    fn list_output_contains_all_agents() {
        let agents = registry::all_agents();
        let names: Vec<&str> = agents.iter().map(|a| a.name()).collect();
        assert!(names.contains(&"kiro"));
        assert!(names.contains(&"claude-code"));
        assert!(names.contains(&"cursor"));
        assert!(names.contains(&"windsurf"));
    }

    #[test]
    fn list_output_contains_all_content() {
        let files = content::all_content();
        let names: Vec<String> = files.iter().map(|f| f.name.clone()).collect();
        assert!(names.contains(&"review-roles.md".to_string()));
        assert!(names.contains(&"prep-review.md".to_string()));
        assert!(names.contains(&"archignore".to_string()));
    }
}
