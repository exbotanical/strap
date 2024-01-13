use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::executor::CommandExecutor;

use super::ProjectName;

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    name: String,
    run: String,
}

impl Step {
    pub fn execute<E: CommandExecutor>(
        &self,
        executor: &E,
        context: &Path,
        project_name: &ProjectName,
    ) -> Result<(), String> {
        let run = self
            .run
            .replace("${{ STRAP_PROJECT_NAME }}", project_name.as_ref()); // project_name will have been validated when we call this

        executor.execute(&run, context)
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        return Self {
            name: "name".into(),
            run: "run".into(),
        };
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use mockall::predicate;

    use crate::executor::MockCommandExecutor;

    use super::*;

    #[test]
    fn execute_step_happy_path() {
        let step = Step {
            name: "test_step".to_string(),
            run: "echo Hello".to_string(),
        };

        let context = PathBuf::from("/some/path");
        let project_name = ProjectName::parse("project".to_string()).unwrap();

        let mut mock_executor = MockCommandExecutor::new();
        mock_executor
            .expect_execute()
            .with(predicate::eq("echo Hello"), predicate::eq(context.clone()))
            .times(1)
            .returning(|_, _| Ok(()));

        let result = step.execute(&mock_executor, &context, &project_name);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn execute_step_interpolates_strapdir() {
        let step = Step {
            name: "test_step".to_string(),
            run: "echo ${{ STRAP_PROJECT_NAME }}".to_string(),
        };

        let context = PathBuf::from("/some/path");
        let project_name = ProjectName::parse("project".to_string()).unwrap();

        let mut mock_executor = MockCommandExecutor::new();
        mock_executor
            .expect_execute()
            .with(
                predicate::eq("echo project"),
                predicate::eq(context.clone()),
            )
            .times(1)
            .returning(|_, _| Ok(()));

        let result = step.execute(&mock_executor, &context, &project_name);

        assert_eq!(result, Ok(()));
    }
}
