use crate::agent::registry;
use crate::error::ZrkError;
use crate::executor::{self, InstallSummary};
use crate::planner;
use crate::util::output;
use crate::Cli;
use std::path::PathBuf;

fn resolve_cwd(cli: &Cli) -> PathBuf {
    cli.cwd.clone().unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

fn resolve_agents(cli: &Cli) -> Result<Vec<Box<dyn crate::agent::Agent>>, ZrkError> {
    if cli.all_targets {
        Ok(registry::all_agents())
    } else {
        match registry::get_agent(&cli.target) {
            Some(agent) => Ok(vec![agent]),
            None => Err(ZrkError::UnknownAgent(cli.target.clone())),
        }
    }
}

fn print_summary(summary: &InstallSummary, quiet: bool) {
    if !quiet {
        println!();
        output::info(&format!(
            "Done: {} installed, {} skipped, {} manual",
            summary.installed, summary.skipped, summary.manual
        ));
    }
}

pub fn run_install(cli: &Cli) -> Result<(), ZrkError> {
    let cwd = resolve_cwd(cli);
    let agents = resolve_agents(cli)?;
    let mut total = InstallSummary::default();

    for agent in &agents {
        if !cli.quiet {
            output::info(&format!("Installing workspace files for {}", agent.label()));
        }
        let actions = planner::plan_install(agent.as_ref(), &cwd, cli.force);
        if cli.dry_run {
            executor::dry_run_display(&actions);
        } else {
            let summary = executor::execute(&actions, cli.quiet)?;
            total.merge(&summary);
        }
    }

    if !cli.dry_run {
        print_summary(&total, cli.quiet);
    }
    Ok(())
}

pub fn run_install_global(cli: &Cli) -> Result<(), ZrkError> {
    let agents = resolve_agents(cli)?;
    let mut total = InstallSummary::default();

    for agent in &agents {
        if !cli.quiet {
            output::info(&format!("Installing global files for {}", agent.label()));
        }
        let actions = planner::plan_install_global(agent.as_ref(), cli.force);
        if cli.dry_run {
            executor::dry_run_display(&actions);
        } else {
            let summary = executor::execute(&actions, cli.quiet)?;
            total.merge(&summary);
        }
    }

    if !cli.dry_run {
        print_summary(&total, cli.quiet);
    }
    Ok(())
}

pub fn run_install_all(cli: &Cli) -> Result<(), ZrkError> {
    let cwd = resolve_cwd(cli);
    let agents = resolve_agents(cli)?;
    let mut total = InstallSummary::default();

    for agent in &agents {
        if !cli.quiet {
            output::info(&format!("Installing all files for {}", agent.label()));
        }
        let actions = planner::plan_install_all(agent.as_ref(), &cwd, cli.force);
        if cli.dry_run {
            executor::dry_run_display(&actions);
        } else {
            let summary = executor::execute(&actions, cli.quiet)?;
            total.merge(&summary);
        }
    }

    if !cli.dry_run {
        print_summary(&total, cli.quiet);
    }
    Ok(())
}
