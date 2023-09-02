use clap::arg;
use clap::Command;
use strap::config::StrapConfig;

// #[test]
// fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::cargo_bin("strap")?;

//     cmd.arg("clibs").arg("test-lib");
//     cmd.assert()
//         .failure()
//         .stderr(predicate::str::contains("could not read file"));

//     Ok(())
// }
