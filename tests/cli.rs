use assert_cmd::Command;

#[test]
fn test_chiru() {
    let mut cmd = Command::cargo_bin("chiru").unwrap();
    cmd.assert().success().stdout("Chiranjeevi Tirunagari!\n");
}

#[test]
fn test_true() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn test_false() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
