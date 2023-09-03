use std::path::Path;
use std::process::Command;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait CommandExecutor {
    fn execute(&self, command: &str, context: &Path) -> Result<(), String>;
}

pub struct RealExecutor;

impl CommandExecutor for RealExecutor {
    fn execute(&self, command: &str, context: &Path) -> Result<(), String> {
        let output = Command::new("sh")
            .current_dir(context)
            .arg("-c")
            .args([command.to_string()])
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}
