use std::{
    env,
    path::{Path, PathBuf},
};

use super::Strap;

#[derive(Debug)]
pub struct StrapContext(PathBuf);

impl StrapContext {
    pub fn parse(strap: &Strap, project_name: &str) -> Result<StrapContext, String> {
        let mut base_path_buf: PathBuf = match &strap.context {
            Some(context) if !context.is_empty() => {
                // TODO: refactor this so we don't have a side-effect in an assignment
                let p = Path::new(context).to_path_buf();
                if !p.is_dir() {
                    println!("Context dir {} doesn't exist. Creating it.", p.display());
                    std::fs::create_dir(&p).map_err(|e| e.to_string())?;
                }
                p
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_with_existing_context() {
        let strap = Strap {
            name: "test_strap".to_string(),
            context: Some("/tmp".to_string()),
            steps: vec![],
        };
        let project_name = "project_name";

        let result = StrapContext::parse(&strap, project_name);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, PathBuf::from("/tmp/project_name"));
    }

    #[test]
    fn parse_without_context() {
        let strap = Strap {
            name: "test_strap".to_string(),
            context: None,
            steps: vec![],
        };
        let project_name = "project_name";

        let result = StrapContext::parse(&strap, project_name);

        // This will check if the resulting path ends with "project_name", because the exact cwd is dynamic.
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.file_name().unwrap(), "project_name");
    }

    #[test]
    fn path_already_exists() {
        let strap = Strap {
            name: "test_strap".to_string(),
            context: Some("/tmp".to_string()),
            steps: vec![],
        };
        let project_name = "existing_path";

        let dir_path = Path::new("/tmp/existing_path");
        if !dir_path.exists() {
            fs::create_dir(dir_path).unwrap();
        }

        let result = StrapContext::parse(&strap, project_name);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cannot create strap test_strap; the path \"/tmp/existing_path\" already exists"
        );

        fs::remove_dir(dir_path).unwrap();
    }
}
