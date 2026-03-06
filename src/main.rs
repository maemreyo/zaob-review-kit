mod agent;
mod commands;
mod content;
mod error;
mod executor;
mod manifest;
mod planner;
mod util;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "zrk",
    about = "Install review workflow files into AI coding agent configs"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Target agent (default: kiro)
    #[arg(long, global = true, default_value = "kiro")]
    pub target: String,

    /// Apply to all supported agents
    #[arg(long, global = true)]
    pub all_targets: bool,

    /// Overwrite existing files
    #[arg(long, global = true)]
    pub force: bool,

    /// Project directory (default: current directory)
    #[arg(long, global = true)]
    pub cwd: Option<PathBuf>,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Suppress non-essential output
    #[arg(long, global = true)]
    pub quiet: bool,

    /// Show what would be done without doing it
    #[arg(long, global = true)]
    pub dry_run: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Install workspace files into current project
    Install,
    /// Install global files into agent's global config
    InstallGlobal,
    /// Install both workspace and global files
    InstallAll,
    /// Force reinstall all with latest content
    Update,
    /// Show installation status
    Status,
    /// List available agents and content files
    List,
    /// Interactive first-time setup wizard
    Init,
    /// Prepare review materials for Claude.ai upload
    Prep(commands::prep::PrepArgs),
}

fn main() {
    let cli = Cli::parse();

    if cli.no_color {
        colored::control::set_override(false);
    }

    let result = match cli.command {
        Command::Install => commands::install::run_install(&cli),
        Command::InstallGlobal => commands::install::run_install_global(&cli),
        Command::InstallAll => commands::install::run_install_all(&cli),
        Command::Update => commands::update::run_update(&cli),
        Command::Status => commands::status::run_status(&cli),
        Command::List => commands::list::run_list(&cli),
        Command::Init => commands::init::run_init(&cli),
        Command::Prep(ref args) => commands::prep::run_prep(&cli, args),
    };

    if let Err(e) = result {
        util::output::error(&format!("{}", e));
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn cli_parse_list() {
        Cli::try_parse_from(["zrk", "list"]).unwrap();
    }

    #[test]
    fn cli_parse_install_with_target() {
        let cli = Cli::try_parse_from(["zrk", "install", "--target", "cursor"]).unwrap();
        assert_eq!(cli.target, "cursor");
    }

    #[test]
    fn cli_parse_install_all_dry_run() {
        let cli = Cli::try_parse_from(["zrk", "install-all", "--dry-run"]).unwrap();
        assert!(cli.dry_run);
    }

    #[test]
    fn cli_parse_prep_git_range() {
        let cli = Cli::try_parse_from(["zrk", "prep", "HEAD~3..HEAD"]).unwrap();
        if let Command::Prep(args) = cli.command {
            assert_eq!(args.scope, vec!["HEAD~3..HEAD"]);
            assert!(args.topic.is_none());
        } else {
            panic!("expected Prep");
        }
    }

    #[test]
    fn cli_parse_prep_commit_hashes() {
        let cli = Cli::try_parse_from(["zrk", "prep", "abc123", "def456"]).unwrap();
        if let Command::Prep(args) = cli.command {
            assert_eq!(args.scope, vec!["abc123", "def456"]);
        } else {
            panic!("expected Prep");
        }
    }

    #[test]
    fn cli_parse_prep_topic() {
        let cli = Cli::try_parse_from(["zrk", "prep", "--topic", "phase-0"]).unwrap();
        if let Command::Prep(args) = cli.command {
            assert_eq!(args.topic.as_deref(), Some("phase-0"));
            assert!(args.scope.is_empty());
        } else {
            panic!("expected Prep");
        }
    }

    #[test]
    fn cli_parse_prep_with_cwd() {
        let cli =
            Cli::try_parse_from(["zrk", "--cwd", "/tmp/project", "prep", "HEAD~1..HEAD"]).unwrap();
        assert!(cli.cwd.is_some());
        if let Command::Prep(args) = cli.command {
            assert_eq!(args.scope, vec!["HEAD~1..HEAD"]);
        } else {
            panic!("expected Prep");
        }
    }
}
