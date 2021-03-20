use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("benfords_law_checker")?;

    cmd.arg("test/file/doesn't/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn file_is_empty() -> Result<(), Box<dyn std::error::Error>> {
    let file = NamedTempFile::new()?;
    let mut cmd = Command::cargo_bin("benfords_law_checker")?;

    cmd.arg(file.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("File doesn't contain any data"));

    Ok(())
}

#[test]
fn invalid_csv_file() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let mut file = NamedTempFile::new()?;
    writeln!(file, "header1,header2\nfoo,bar\nquux,baz,foobar")?;
    let mut cmd = Command::cargo_bin("benfords_law_checker")?;

    cmd.arg(file.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("CSV error caused by"));

    Ok(())
}

#[test]
fn no_required_header() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let mut file = NamedTempFile::new()?;
    writeln!(file, "header1,header2\nfoo,bar\nquux,baz")?;
    let mut cmd = Command::cargo_bin("benfords_law_checker")?;

    cmd.arg(file.path()).arg("-i header3");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Couldn't find required header"));

    Ok(())
}
