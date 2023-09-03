use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use serde::{Deserialize, Serialize};

use crate::util::{has_duplicates, read_file_string};

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    name: Option<String>,
    uses: Option<String>,
    run: Option<String>,
}

impl Step {
    pub fn execute(&self, context: &PathBuf) -> Result<(), String> {
        let cmd = self
            .run
            .to_owned() //TODO:
            // TODO: validate steps[n].run
            .expect(format!("run specifier required for step {}", "TODO: self.name").as_str());

        // TODO: windows?
        let output = Command::new("sh")
            .current_dir(context)
            .arg("-c")
            .args([cmd])
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Strap {
    pub name: String,
    pub context: Option<String>,
    pub steps: Vec<Step>,
}

// pub struct StrapIter<'a> {
//     strap: &'a Strap,
//     step_cursor: usize,
// }

// impl Strap {
//     pub fn iter(&self) -> StrapIter {
//         StrapIter {
//             strap: self,
//             step_cursor: 0,
//         }
//     }
// }

// impl<'a> Iterator for StrapIter<'a> {
//     type Item = Result<(), String>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.step_cursor < self.strap.steps.len() {
//             let result = self.strap.steps[self.step_cursor].execute();
//             self.step_cursor += 1;
//             Some(result)
//         } else {
//             None
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrapConfig {
    pub straps: Vec<Strap>,
}

const DEFAULT_CONFIG_DIR: &str = "./tests/fixtures/valid_config.yaml";

impl StrapConfig {
    pub fn parse(config_path: Option<&str>) -> Result<StrapConfig, String> {
        let config_as_str = read_file_string(config_path.unwrap_or(DEFAULT_CONFIG_DIR))
            .map_err(|e| e.to_string())
            .unwrap();

        let config: StrapConfig = match serde_yaml::from_str(&config_as_str) {
            Ok(conf) => conf,
            Err(e) => return Err(e.to_string()),
        };

        config.validate()?;

        Ok(config)
    }

    pub fn find_strap(&self, strap_name: &str) -> Result<&Strap, String> {
        let strap = self
            .straps
            .iter()
            .find(|strap| strap.name.eq(strap_name))
            .ok_or_else(|| format!("No strap found with name: {}", strap_name))?;

        Ok(strap)
    }

    fn validate(&self) -> Result<(), String> {
        let strap_ids: Vec<&String> = self.straps.iter().map(|strap| &strap.name).collect();
        if has_duplicates(&strap_ids) {
            Err("config contains duplicate straps".to_string())
        } else {
            Ok(())
        }
    }
}

// Validate
// 1. Every strap has expected args run
// 2. If strap has no context, use curre
