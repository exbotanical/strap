use serde::{Deserialize, Serialize};

use crate::{
    domain::Strap,
    fs::FileReader,
    util::{expand_tilde_if_extant, has_duplicates},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct StrapConfig {
    pub straps: Vec<Strap>,
}
// TODO: allow yml, yaml
// TODO: use std config dir
const DEFAULT_CONFIG_DIR: &str = "~/.config/strap-config.yaml";

impl StrapConfig {
    pub fn parse<R: FileReader>(
        reader: &R,
        config_path: Option<&str>,
    ) -> Result<StrapConfig, String> {
        let config_as_str = reader
            .read_file_string(
                config_path.unwrap_or(
                    // Let it fail if we can't resolve; at this point our fallback has failed and we can't do anything else
                    expand_tilde_if_extant(DEFAULT_CONFIG_DIR)
                        .unwrap()
                        .to_str()
                        .unwrap(),
                ),
            )
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

#[cfg(test)]
mod tests {
    use crate::fs::MockFileReader;

    use super::*;
    use mockall::predicate;

    #[test]
    fn parse_config_happy_path() {
        let mut mock_reader = MockFileReader::new();

        let mock_content = r#"
        straps:
          - name: "test_strap"
            steps:
              - name: "step1"
                run: "echo hello"
        "#;

        mock_reader
            .expect_read_file_string()
            .with(predicate::eq("./tests/fixtures/valid_config.yaml"))
            .return_once(move |_| Ok(mock_content.to_string()));

        let config = StrapConfig::parse(&mock_reader, Some("./tests/fixtures/valid_config.yaml"));

        assert!(config.is_ok());
    }

    #[test]
    fn parse_invalid_config() {
        let mut mock_reader = MockFileReader::new();
        mock_reader
            .expect_read_file_string()
            .with(predicate::eq("./tests/fixtures/invalid_config.yaml"))
            .return_once(move |_| Ok("invalid content".to_string()));

        let config = StrapConfig::parse(&mock_reader, Some("./tests/fixtures/invalid_config.yaml"));

        assert!(config.is_err());
    }

    #[test]
    fn find_existing_strap() {
        let mock_content = r#"
        straps:
          - name: "test_strap"
            steps:
              - name: "step1"
                run: "echo hello"
        "#;

        let config: StrapConfig = serde_yaml::from_str(mock_content).unwrap();
        let strap = config.find_strap("test_strap");

        assert!(strap.is_ok());
    }

    #[test]
    fn find_non_existing_strap() {
        let mock_content = r#"
        straps:
          - name: "test_strap"
            steps:
              - name: "step1"
                run: "echo hello"
        "#;

        let config: StrapConfig = serde_yaml::from_str(mock_content).unwrap();
        let strap = config.find_strap("non_existing_strap_name");

        assert!(strap.is_err());
    }
}
