use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn zrk() -> Command {
    Command::cargo_bin("zrk").unwrap()
}

#[test]
fn status_after_install_shows_success() {
    let dir = TempDir::new().unwrap();

    // Install first
    zrk()
        .args(["install", "--target", "kiro", "--cwd", dir.path().to_str().unwrap()])
        .assert()
        .success();

    // Check status
    zrk()
        .args(["status", "--target", "kiro", "--cwd", dir.path().to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn list_shows_agents_and_content() {
    zrk()
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Kiro"))
        .stdout(predicate::str::contains("Claude Code"))
        .stdout(predicate::str::contains("Cursor"))
        .stdout(predicate::str::contains("Windsurf"))
        .stdout(predicate::str::contains("review-roles.md"));
}
