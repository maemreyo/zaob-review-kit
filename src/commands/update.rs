use crate::error::ZrkError;
use crate::Cli;

pub fn run_update(cli: &Cli) -> Result<(), ZrkError> {
    // Update is just install-all with force=true
    let force_cli = Cli {
        command: crate::Command::Update,
        target: cli.target.clone(),
        all_targets: cli.all_targets,
        force: true, // always force
        cwd: cli.cwd.clone(),
        no_color: cli.no_color,
        quiet: cli.quiet,
        dry_run: cli.dry_run,
    };
    // We can't mutate cli, so we construct a new one with force=true
    crate::commands::install::run_install_all(&force_cli)
}
