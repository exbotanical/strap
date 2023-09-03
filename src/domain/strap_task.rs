use crate::{config::Strap, domain::ProjectName};

use super::StrapContext;

pub struct StrapTask<'a> {
    pub strap: &'a Strap,
    pub project_name: ProjectName,
    pub context: StrapContext,
}

impl<'a> StrapTask<'a> {
    pub fn new(strap: &'a Strap, project_name: ProjectName) -> Result<StrapTask, String> {
        let context = StrapContext::parse(strap, project_name.as_ref())?;

        Ok(StrapTask {
            strap,
            project_name,
            context,
        })
    }

    pub fn execute_steps(&self) -> Result<(), String> {
        for step in self.strap.steps.iter() {
            step.execute(self.context.as_ref())?;
        }

        Ok(())
    }
}
