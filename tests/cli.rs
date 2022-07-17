use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn should_exit_with_error_when_file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("benfords_law_checker")?;

    cmd.arg("test/file/doesn't/exist");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn should_exit_with_error_file_is_empty() -> Result<(), Box<dyn std::error::Error>> {
    let file = NamedTempFile::new()?;
    let mut cmd = Command::cargo_bin("benfords_law_checker")?;

    cmd.arg(file.path());
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("File doesn't contain any data"));

    Ok(())
}

#[test]
fn should_exit_with_error_invalid_csv_file() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let mut file = NamedTempFile::new()?;
    writeln!(file, "header1,header2\nfoo,bar\nquux,baz,foobar")?;
    let mut cmd = Command::cargo_bin("benfords_law_checker")?;

    cmd.arg(file.path());
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("CSV error caused by"));

    Ok(())
}

#[test]
fn should_exit_with_error_no_required_header() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let mut file = NamedTempFile::new()?;
    writeln!(file, "header1,header2\nfoo,bar\nquux,baz")?;
    let mut cmd = Command::cargo_bin("benfords_law_checker")?;

    cmd.arg(file.path()).arg("-i header3");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Couldn't find required header"));

    Ok(())
}
