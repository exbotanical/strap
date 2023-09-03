use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::executor::CommandExecutor;

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    name: String,
    uses: Option<String>,
    run: String,
}

impl Step {
    pub fn execute<E: CommandExecutor>(&self, context: &Path, executor: &E) -> Result<(), String> {
        executor.execute(&self.run, context)
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        return Self {
            name: "name".into(),
            uses: None,
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
    fn test_execute_step() {
        let step = Step {
            name: "test_step".to_string(),
            uses: None,
            run: "echo Hello".to_string(),
        };

        let context = PathBuf::from("/some/path");

        let mut mock_executor = MockCommandExecutor::new();
        mock_executor
            .expect_execute()
            .with(predicate::eq("echo Hello"), predicate::eq(context.clone()))
            .times(1)
            .returning(|_, _| Ok(()));

        let result = step.execute(&context, &mock_executor);

        assert_eq!(result, Ok(()));
    }
}
