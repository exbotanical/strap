use std::{
    env,
    path::{Path, PathBuf},
};

use crate::config::Strap;

#[derive(Debug)]
pub struct StrapContext(PathBuf);

impl StrapContext {
    pub fn parse(strap: &Strap, project_name: &str) -> Result<StrapContext, String> {
        let mut base_path_buf: PathBuf = match &strap.context {
            Some(context) if !context.is_empty() => Path::new(context).to_path_buf(),
            _ => {
                let cwd = env::current_dir().unwrap();
                println!("No context set for {}. Assuming cwd as context", strap.name);
                cwd
            }
        };

        // TODO: allow custom
        base_path_buf.push(project_name);

        if base_path_buf.to_str().is_none() {
            return Err("Path is not valid UTF-8.".to_string());
        }

        if base_path_buf.exists() {
            return Err(format!(
                "Cannot create strap {}; the path {:?} already exists",
                strap.name, base_path_buf
            ));
        }

        Ok(Self(base_path_buf))
    }
}

impl AsRef<PathBuf> for StrapContext {
    fn as_ref(&self) -> &PathBuf {
        &self.0
    }
}
