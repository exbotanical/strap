use crate::executor::CommandExecutor;

use super::{ProjectName, Strap, StrapContext};

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

    pub fn execute_steps<E: CommandExecutor>(&self, executor: &E) -> Result<(), String> {
        for step in self.strap.steps.iter() {
            step.execute(self.context.as_ref(), executor)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;

    use super::*;
    use crate::{domain::Step, executor::MockCommandExecutor};

    #[test]
    fn test_execute_steps_happy_case() {
        let strap = Strap {
            name: "test_strap".to_string(),
            context: Some("/tmp".to_string()),
            steps: vec![Step::mock(), Step::mock(), Step::mock()],
        };

        let project_name = ProjectName::parse("project_name".into()).expect("testing bug");
        let task = StrapTask::new(&strap, project_name).unwrap();

        let mut mock_executor = MockCommandExecutor::new();

        mock_executor
            .expect_execute()
            .with(predicate::always(), predicate::always())
            .times(3)
            .returning(|_, _| Ok(()));

        let result = task.execute_steps(&mock_executor);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_execute_steps_stops_on_error() {
        let strap = Strap {
            name: "test_strap".to_string(),
            context: Some("/tmp".to_string()),
            steps: vec![Step::mock(), Step::mock(), Step::mock()],
        };

        let project_name = ProjectName::parse("project_name".into()).expect("testing bug");
        let task = StrapTask::new(&strap, project_name).unwrap();

        let mut mock_executor = MockCommandExecutor::new();

        // The first execution succeeds
        mock_executor
            .expect_execute()
            .with(predicate::always(), predicate::always())
            .times(1)
            .returning(|_, _| Ok(()));

        // The second one fails
        mock_executor
            .expect_execute()
            .with(predicate::always(), predicate::always())
            .times(1)
            .returning(|_, _| Err("ERROR".to_string()));

        // Ensure the third one is never called
        mock_executor
            .expect_execute()
            .with(predicate::always(), predicate::always())
            .times(0)
            .returning(|_, _| Ok(()));

        let result = task.execute_steps(&mock_executor);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "ERROR");
    }
}
