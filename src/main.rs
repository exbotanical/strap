use strap::cli::{cli, config_cli};
use strap::config::StrapConfig;
use strap::domain::{ProjectName, StrapTask};
use strap::error::StrapError;
use strap::executor::RealExecutor;
use strap::fs::RealFileReader;

// TODO: store config hash
// on command, compare hash to stored
// if change, load the config
fn main() -> Result<(), StrapError> {
    let cmd = config_cli();
    let matches = cmd.get_matches();

    // Get the config path, if specified
    let config_path = matches.get_one::<String>("config").map(|s| s.as_str());
    // Parse the config. If no path specified by user, this will use the default path
    let config = StrapConfig::parse(&RealFileReader, config_path)?;

    // Patch on dynamically computed cli using config
    if let Some(matches) = cli(&config).get_matches().subcommand() {
        // Find the strap, throw if not found (see weird cli workaround TODO: write about it)
        let strap = config.find_strap(matches.0)?;

        // If match, get the first arg, which is always the project name
        let project_name = ProjectName::try_from(matches)?;

        // Build a task to represent the work we're going to do
        let strap_task = StrapTask::new(strap, project_name)?;

        // Create the new project dir for the strap
        std::fs::create_dir(strap_task.context.as_ref())?;

        // Execute the strap steps
        strap_task.execute_steps(&RealExecutor)?;
    }

    Ok(())
}
