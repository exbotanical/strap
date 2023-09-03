use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::{tempdir, NamedTempFile, TempDir};

fn setup_config() -> (NamedTempFile, TempDir) {
    let temp_dir = tempdir().unwrap();

    let config_content = format!(
        r#"
        straps:
          - name: "clib"
            context: {}
            steps:
              - name: "step1"
                run: "echo hello"
        "#,
        temp_dir.path().display()
    );

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), config_content).unwrap();
    (temp_file, temp_dir)
}

#[test]
fn no_strap_matched() {
    let (config_file, _) = setup_config();

    let mut cmd = Command::cargo_bin("strap").unwrap();
    cmd.arg("--config")
        .arg(config_file.path().to_str().unwrap())
        .arg("noexist");
    let result = cmd.output().unwrap();

    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("No strap found"));
    // TODO: error msg fmt str consts
}

#[test]
fn no_project_name_arg() {
    let (config_file, _) = setup_config();

    let mut cmd = Command::cargo_bin("strap").unwrap();
    cmd.arg("--config")
        .arg(config_file.path().to_str().unwrap())
        .arg("clib");
    let result = cmd.output().unwrap();

    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr)
        .contains("project_name not specified but was required"));
    // TODO: error msg fmt str consts
}

#[test]
fn project_name_and_context_path_exists() {
    let (config_file, temp_dir) = setup_config();
    let project_name = "test_name";
    let full_path = temp_dir.path().join(project_name);

    // Create the dir as a subdir of the tmp dir used for the context so we know it exists
    fs::create_dir(full_path).unwrap();

    let mut cmd = Command::cargo_bin("strap").unwrap();
    cmd.arg("--config")
        .arg(config_file.path().to_str().unwrap())
        .arg("clib")
        .arg(project_name);
    let result = cmd.output().unwrap();

    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("Cannot create strap clib",));
    // TODO: assert on exact match
    // TODO: error msg fmt str consts
}

// #[test]
// fn test_valid_strap_execution() {
//     let config_file = setup_config();

//     let dir = tempdir().unwrap();
//     let path = dir.path().join("clib");

//     let mut cmd = Command::cargo_bin("strap").unwrap();
//     cmd.arg("--config")
//         .arg(config_file.path().to_str().unwrap())
//         .arg("clib")
//         .arg(path.to_str().unwrap());
//     let result = cmd.output().unwrap();

//     assert!(result.status.success());
//     assert!(path.exists());
// }

// ... More tests ...
