use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn no_arg_provided() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("yamlidate")?;
    cmd.assert()
        .stderr(predicate::str::contains("No file provided as argument."));
    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("yamlidate")?;
    cmd.arg("test/file/doesnt/exist");
    cmd.assert()
        .stderr(predicate::str::contains("Error reading file"));
    Ok(())
}

#[test]
fn yaml_is_not_valid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("yamlidate")?;
    cmd.arg("invalid_yaml.yaml");
    cmd.assert()
        .stderr(predicate::str::contains("The YAML file is not valid"));
    Ok(())
}

#[test]
fn yaml_is_valid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("yamlidate")?;
    cmd.arg("valid_yaml.yaml");
    cmd.assert()
        .stdout(predicate::str::contains("The YAML file is valid !"));
    Ok(())
}
