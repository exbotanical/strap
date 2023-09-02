use strap::cli::{cli, config_cli};
use strap::config::StrapConfig;
use strap::error::StrapError;

// TODO: store config hash
// on command, compare hash to stored
// if change, load the config
fn main() -> Result<(), StrapError> {
    let cmd = config_cli();
    let matches = cmd.get_matches();

    let config_path = matches.get_one::<String>("config").map(|s| s.as_str());
    let config = StrapConfig::parse(config_path)?;

    if let Some(matches) = cli(&config).get_matches().subcommand() {
        let strap = config.find_strap(matches.0)?;

        let project_name = matches
            .1
            .get_one::<String>("project_name")
            .expect("required");

        let base_path_buf = strap.get_valid_context(matches.0, project_name)?;

        std::fs::create_dir(&base_path_buf)?;

        for res in strap.iter() {
            println!("{:#?}", res);
            res?;
        }
    }

    Ok(())
}

// wl-clipboard
