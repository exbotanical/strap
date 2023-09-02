use clap::{arg, Command};

use crate::config::StrapConfig;

pub fn config_cli() -> Command {
    let cmd = Command::new("stack")
        .about("TODO:")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .ignore_errors(true)
        .arg(arg!(-c --config <FILE> "Sets a custom config file"));

    cmd
}

pub fn cli(config: &StrapConfig) -> Command {
    let mut cmd = Command::new("stack")
        .about("TODO:")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .ignore_errors(true);

    let strap_ids: Vec<String> = config
        .straps
        .iter()
        .map(|strap| strap.name.clone())
        .collect();

    for key in &strap_ids {
        cmd = cmd.subcommand(Command::new(key).arg(arg!(project_name: [PROJECT_NAME])));
    }

    cmd
}
