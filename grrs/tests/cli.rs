use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertion
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    
    cmd.arg("foobar").arg("test/file/mek");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

use assert_fs::prelude::*;

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("hello mekness\n my old\nmek")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("mek").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello mekness\nmek"));

    Ok(())
}