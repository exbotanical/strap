use std::env::Args;

use super::{ProjectName, StrapName};

#[derive(Debug)]
pub struct StrapArgs {
    // pub strap_config: StrapConfigPath,
    pub strap_name: StrapName,
    pub project_name: ProjectName,
}

impl TryFrom<Args> for StrapArgs {
    type Error = String;

    fn try_from(mut value: Args) -> Result<Self, Self::Error> {
        let strap_name_arg = value.nth(1).expect("A strap name is required");
        let project_name_arg = value.next().expect("A project name is required");
        let strap_name = StrapName::parse(strap_name_arg.clone())?;
        let project_name = ProjectName::parse(project_name_arg)?;

        Ok(StrapArgs {
            strap_name,
            project_name,
        })
    }
}
